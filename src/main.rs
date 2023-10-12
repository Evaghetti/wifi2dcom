use clap::Parser;
use cli::Wifi2DCom;

mod arduino;
mod cli;

fn main() {
    let args = Wifi2DCom::parse();
    let config = args.get_config().expect("Not able to configure app");
    println!("{:?}", config);
}
