use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use tokio::net::TcpStream;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Mutex;

use crate::{Configuration, establish_connection};

/// Flasher server structure
#[derive(Debug)]
pub struct Server {
	/// Server configuration
	pub config: Configuration,
	/// Server address
	pub address: SocketAddr,
	/// Server runtime
	pub runtime: Runtime,
	/// Database used for flasher
	pub database: DatabaseConnection,
	/// Connections to friends
	pub friends: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl Server {
	/// Create new server instance
	pub async fn new(config: &Configuration) -> Server {
		// Creating address from server.listen
		let address = SocketAddr::new(
			config.server.listen.host.parse().unwrap(),
			config.server.listen.port
		);
		// Creating database connection
		let database = establish_connection(&config).await;
		// Creating runtime builder
		let runtime = Builder::new_multi_thread().build().unwrap();

		Server {
			config: config.clone(),
			address,
			database,
			runtime,
			friends: Arc::new(Mutex::new(HashMap::new()))
		}
	}

	/// Run server
	pub fn run(&self) {
		println!("{:#?}", self);
		if self.config.server.listen.tcp {
			if let Err(e) = self.start_tcp_server() {
				println!("Failed to start TCP listener on {}: {:?}", self.address, e);
			}
		}

		if self.config.server.listen.udp {
			if let Err(e) = self.start_udp_server() {
				println!("Failed to start UDP listener on {}: {:?}", self.address, e);
			}
		}
	}

	fn start_udp_server(&self) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn start_tcp_server(&self) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
