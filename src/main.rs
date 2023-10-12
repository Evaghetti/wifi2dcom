use std::{thread, time::Duration};

fn main() {
    // Open port
    let mut port = serialport::new("/dev/ttyUSB0", 9_600)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    thread::sleep(Duration::from_secs(1));

    // Write data
    let output = "V1-FC03\n".as_bytes();
    let written_bytes = port.write(output).expect("Write failed!");
    println!("Written bytes len = {}", written_bytes);
    println!("Written bytes = {:?}", output);

    // Wait for data
    loop {
        let available_bytes: u32 = port.bytes_to_read().expect("Failed to read buff size");
        if available_bytes > 0 {
            break;
        }
        println!("No data");
    }

    // Read data
    let mut serial_buf: Vec<u8> = vec![0; written_bytes];
    port.read_exact(serial_buf.as_mut_slice())
        .expect("Found no data!");
    println!("Received data{:?}", serial_buf);
}
