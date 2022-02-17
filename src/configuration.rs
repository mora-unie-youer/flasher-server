use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
	#[serde(rename = "type")]
	pub db_type: String,
	#[serde(default = "default_database_host")]
	pub host: String,

	pub username: String,
	pub password: String,

	#[serde(rename = "name", default = "default_database_name")]
	pub db_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerListenConfiguration {
	#[serde(default = "default_server_listen_host")]
	pub host: String,
	#[serde(default = "default_server_listen_port")]
	pub port: u16,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerConfiguration {
	#[serde(default)]
	pub listen: ServerListenConfiguration,

	#[serde(default = "default_server_friends")]
	pub friends: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	#[serde(default)]
	pub database: DatabaseConfiguration,
	#[serde(default)]
	pub server: ServerConfiguration,
}

impl Configuration {
	pub fn config_file(filename: &str) -> Option<Configuration> {
		// Checking if file can be opened
		match File::open(filename) {
			Ok(mut file) => {
				// Reading text from file
				let mut text = String::new();
				file.read_to_string(&mut text).unwrap();
				// Parsing toml
				if let Ok(settings) = toml::from_str(&text) {
					return Some(settings);
				}
				None
			},
			Err(..) => None,
		}
	}
}

fn default_database_host() -> String {
	String::from("localhost")
}

fn default_database_name() -> String {
	String::from("flasher")
}

fn default_server_listen_host() -> String {
	String::from("[::1]")
}

fn default_server_listen_port() -> u16 {
	6666
}

fn default_server_friends() -> Vec<String> {
	vec![]
}
