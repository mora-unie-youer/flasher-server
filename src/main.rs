use std::error::Error;

use flasher_server::Settings;

const DEFAULT_CONFIG_FILE: &str = "/etc/flasher.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = Settings::config_file(DEFAULT_CONFIG_FILE)
		.unwrap_or_else(|| panic!("Couldn't read config file"));
	println!("{:?}", config);
	Ok(())
}
