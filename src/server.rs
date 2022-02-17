use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use tokio::net::{TcpStream, TcpListener};
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

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
	pub async fn run(&mut self) {
		let mut handles = vec![];

		if self.config.server.listen.tcp {
			let handle = self.start_tcp_server().await;
			handles.push(
				handle.unwrap_or_else(|e| panic!("Failed to start TCP listener on {}: {:?}", self.address, e))
			);
		}

		if self.config.server.listen.udp {
			if let Err(e) = self.start_udp_server().await {
				println!("Failed to start UDP listener on {}: {:?}", self.address, e);
			}
		}

		futures::future::join_all(handles).await;
	}

	async fn start_tcp_server(&mut self) -> Result<JoinHandle<()>, Box<dyn Error>> {
		// Creating TCP listener
		let socket = TcpListener::bind(self.address).await?;
		// Creating TCP listener thread
		Ok(self.runtime.spawn(async move {
			loop {
				let (stream, addr) = socket.accept().await.unwrap();
			}
		}))
	}

	async fn start_udp_server(&self) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
