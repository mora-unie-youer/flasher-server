use std::env;
use std::error::Error;
use std::process::exit;

use getopts::Options;

use flasher_server::{establish_connection, Settings};

const DEFAULT_CONFIG_FILE: &str = "/etc/flasher.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let program = &args[0];

	let mut opts = Options::new();
	opts.optflag("h", "help", "Print help message");
	opts.optopt("c", "config", "Path to configuration file", "FILE");
	let opt_matches = opts.parse(&args[1..]).expect("Couldn't parse arguments");

	if opt_matches.opt_present("h") {
		let brief = format!("usage: {} [options]", program);
		println!("{}", opts.usage(&brief));
		exit(0);
	}

	let config_file = opt_matches.opt_str("c")
		.unwrap_or(DEFAULT_CONFIG_FILE.to_string());

	let config = Settings::config_file(&config_file)
		.unwrap_or_else(|| panic!("Couldn't read config file"));

	let database = establish_connection(config).await;
	println!("{:?}", database);

	Ok(())
}
