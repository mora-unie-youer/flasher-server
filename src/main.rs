use std::env;
use std::error::Error;

use getopts::Options;

use flasher_server::Settings;

const DEFAULT_CONFIG_FILE: &str = "/etc/flasher.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	let mut opts = Options::new();
	opts.optopt("c", "config", "Configuration file", "FILE");

	let opt_matches = opts.parse(&args[1..]).expect("Couldn't parse arguments");

	let config_file = opt_matches.opt_str("c")
		.unwrap_or(DEFAULT_CONFIG_FILE.to_string());

	let config = Settings::config_file(&config_file)
		.unwrap_or_else(|| panic!("Couldn't read config file"));
	println!("{:?}", config);
	Ok(())
}
