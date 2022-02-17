use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseSettings {
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
pub struct ServerSettings {
	#[serde(default = "default_server_listen")]
	pub listen: String,

	#[serde(default = "default_server_friends")]
	pub friends: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
	#[serde(default)]
	pub database: DatabaseSettings,
	#[serde(default)]
	pub server: ServerSettings,
}

impl Settings {
	pub fn config_file(filename: &str) -> Option<Settings> {
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

fn default_server_listen() -> String {
	String::from("[::1]:6666")
}

fn default_server_friends() -> Vec<String> {
	vec![]
}
