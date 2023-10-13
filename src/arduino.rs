use anyhow::{Context, Result};
use serialport::SerialPort;
use std::{thread, time::Duration};

fn send_command(port: &mut Box<dyn SerialPort>, data: &str) -> Result<()> {
    let output = format!("{}\n", data);
    let written_bytes = port
        .write(output.as_bytes())
        .with_context(|| format!("Failed to send {:?} to D-COM", output))?;

    println!("Sent to D-COM '{:?}', {} bytes", output, written_bytes);
    Ok(())
}

fn receive_packet(port: &mut Box<dyn SerialPort>, count_bytes: usize) -> Result<String> {
    let mut serial_buf: Vec<u8> = vec![0; count_bytes];
    port.read_exact(serial_buf.as_mut_slice())
        .expect("Found no data!");

    let converted_buf =
        String::from_utf8(serial_buf).with_context(|| "Received broken characters")?;
    Ok(converted_buf)
}

fn receive_response(port: &mut Box<dyn SerialPort>) -> Result<String> {
    let mut result = "".to_string();

    loop {
        let available_bytes: usize =
            port.bytes_to_read().expect("Failed to read buff size") as usize;

        if available_bytes == 0 {
            continue;
        }

        let current_packet = receive_packet(port, available_bytes)?;

        result += &current_packet;
        if current_packet.ends_with("\r\n") {
            break;
        }
    }

    Ok(result.trim().to_string())
}

pub fn get_dcom_output(serial_port: &str, digirom: &str) -> Result<String> {
    // Open port
    let mut port = serialport::new(serial_port, 9_600)
        .timeout(std::time::Duration::from_millis(1000))
        .flow_control(serialport::FlowControl::Hardware)
        .open()
        .with_context(|| format!("Not possible to open port {}", serial_port))?;

    thread::sleep(Duration::from_secs(5));

    // Clearing both buffer to not get garbage
    port.clear(serialport::ClearBuffer::All)?;

    // Write data
    send_command(&mut port, digirom)?;

    // Wait for data
    loop {
        let received_line = receive_response(&mut port)?;
        println!("Received {}", received_line);
        if received_line.len() == 0
            || received_line.starts_with("got")
            || received_line.ends_with("t")
        {
            println!("Skipping");
            continue;
        }

        send_command(&mut port, "Random stuff")?;
        let _ = receive_response(&mut port)?;
        // Read data
        return Ok(received_line);
    }
}
