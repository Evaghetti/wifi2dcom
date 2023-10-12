mod arduino;

use arduino::get_dcom_output;

fn main() {
    get_dcom_output("/dev/ttyUSB0", "V1-FC03-FD02").expect("Failed to send and read");
}
