use anyhow::{anyhow, Result};
use clap::{Args, Parser};
use clio::Input;
use serde::Deserialize;

#[derive(Deserialize, Args, Debug, Clone)]
#[group(id = "inline_config", conflicts_with = "file_config")]
pub struct InlineWificomConfig {
    /// 'username' config given by the secrets.py
    #[arg(short, long)]
    pub username: String,

    /// 'password' config given by the secrets.py
    #[arg(short, long)]
    pub password: String,

    /// 'user_uuid' config given by the secrets.py
    #[arg(short = 'i', long)]
    pub user_uuid: String,

    /// 'device_uuid' config given by the secrets.py
    #[arg(short, long)]
    pub device_uuid: String,
}

#[derive(Parser)]
#[command(name = "wifi2dcom", about = "Use your D-COM as if it was a wificom!")]
pub struct Wifi2DCom {
    /// Which serial port your D-COM is connected
    #[arg(short, long)]
    pub serial_port: String,

    /// JSON file with configs given by wificom.dev
    #[arg(
        short,
        long,
        group = "file_config",
        conflicts_with = "inline_config",
        required = true
    )]
    config_file: Option<Input>,

    #[command(flatten)]
    wificom_config: Option<InlineWificomConfig>,
}

impl Wifi2DCom {
    pub fn get_config(&self) -> Result<InlineWificomConfig> {
        if let Some(config) = self.wificom_config.as_ref() {
            Ok(config.to_owned())
        } else if let Some(config_file) = self.config_file.as_ref() {
            let mut file_reader = config_file.clone();
            let file_data: InlineWificomConfig = serde_json::from_reader(&mut file_reader)?;

            Ok(file_data)
        } else {
            Err(anyhow!("No valid wificom config given"))
        }
    }
}
