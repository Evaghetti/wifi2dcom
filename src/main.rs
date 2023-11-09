mod arduino;
mod cli;

use anyhow::{Context, Result};
use async_std::sync::Mutex;
use clap::Parser;
use cli::Wifi2DCom;
use futures::{executor::block_on, stream::StreamExt};
use mqtt::Message;
use paho_mqtt as mqtt;
use serde::{Deserialize, Serialize};
use serde_this_or_that::as_u64;
use std::{sync::Arc, time::Duration};

use crate::arduino::get_dcom_output;

const WIFICOM_URL: &'static str = "mqtt://mqtt.wificom.dev:1883";

#[derive(Serialize, Deserialize)]
struct BattleRequest {
    digirom: String,
    #[serde(deserialize_with = "as_u64")]
    application_id: u64,
    hide_output: bool,
    api_response: bool,
    ack_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BattleResponse {
    application_uuid: u64,
    device_uuid: String,
    output: String,
    ack_id: String,
}

fn main() -> Result<()> {
    let args = Wifi2DCom::parse();
    let config = args.get_config()?;

    let input_topic = format!(
        "{}/f/{}-{}/wificom-input",
        config.username, config.user_uuid, config.device_uuid
    );

    let output_topic = format!(
        "{}/f/{}-{}/wificom-output",
        config.username, config.user_uuid, config.device_uuid
    );

    println!("Connecting to the MQTT server at '{}'...", WIFICOM_URL);

    let create_opts = mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(WIFICOM_URL)
        .client_id("rust_async_subscribe")
        .finalize();

    let mut cli = mqtt::AsyncClient::new(create_opts)
        .with_context(|| format!("Error creating the client"))?;

    block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Create the connect options, explicitly requesting MQTT v3.x
        let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
            .keep_alive_interval(Duration::from_secs(30))
            .clean_session(true)
            .user_name(config.username)
            .password(config.password)
            .finalize();

        // Make the connection to the broker
        cli.connect(conn_opts).await?;

        cli.subscribe(input_topic, 0).await?;

        let heartbeat_cli = Arc::new(Mutex::new(()));
        let cli_clone = cli.clone();
        let output_topic_clone = output_topic.clone();
        let heartbeat_cli_clone = Arc::new(Mutex::new(()));
        let device_uuid_clone = config.device_uuid.clone();
        async_std::task::spawn(async move {
            loop {
                let _lock = heartbeat_cli_clone.lock();

                let battle_response = BattleResponse {
                    application_uuid: 0,
                    device_uuid: device_uuid_clone.clone(),
                    output: "None".to_string(),
                    ack_id: "heartbeat".to_string(),
                };

                let msg = Message::new(
                    &output_topic_clone,
                    serde_json::to_string(&battle_response)
                        .expect("Not possible to create payload of heartbeat")
                        .as_bytes(),
                    mqtt::QOS_0,
                );
                println!("Sending {} as heartbeat", msg);
                cli_clone
                    .publish(msg)
                    .await
                    .expect("Not able to send heartbeat");

                std::mem::drop(_lock);
                async_std::task::sleep(Duration::from_secs(20)).await;
            }
        });

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                let _lock = heartbeat_cli.lock();

                println!("Received {}", msg);
                let battle_request: BattleRequest = serde_json::from_str(&msg.payload_str())?;
                let battle_response = BattleResponse {
                    application_uuid: battle_request.application_id,
                    device_uuid: config.device_uuid.clone(),
                    output: get_dcom_output(&args.serial_port, &battle_request.digirom)?,
                    ack_id: battle_request.ack_id.unwrap_or("".to_string()),
                };

                let msg = Message::new(
                    &output_topic,
                    serde_json::to_string(&battle_response)?.as_bytes(),
                    mqtt::QOS_0,
                );

                println!("Sent {}", msg);

                cli.publish(msg).await?;
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    println!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        // Explicit return type for the async block
        Ok::<(), anyhow::Error>(())
    })
    .with_context(|| "Error running mqtt client")?;
    Ok(())
}
