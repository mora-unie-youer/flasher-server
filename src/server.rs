use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use tokio::net::{TcpStream, TcpListener};
use tokio::task::JoinHandle;

use crate::{establish_connection, Configuration};

/// TCP server
pub struct TcpServer {
	/// Server address
	pub address: SocketAddr,
	/// Server configuration
	pub config: Arc<Configuration>,
	/// Database
	pub database: DatabaseConnection
}

impl TcpServer {
	pub async fn new(config: Arc<Configuration>) -> TcpServer {
		// Creating server address using config.server.listen
		let address = SocketAddr::new(
			config.server.listen.host.parse().unwrap(),
			config.server.listen.port
		);
		// Creating database connection
		let database = establish_connection(&config).await;
		// Creating server
		TcpServer {
			address,
			config: config.clone(),
			database
		}
	}

	pub async fn start(self) -> Result<JoinHandle<()>, Box<dyn Error>> {
		// Creating TCP socket
		let socket = TcpListener::bind(self.address).await?;
		// Creating TCP listener
		let handle = tokio::spawn(async move {
			// Listening for all connections
			loop {
				// Accepting connections
				let (mut stream, addr) = socket.accept().await.unwrap();
				println!("Accepted connection: {:?} - {:?}", addr, stream);
				// Creating a new buffer for client
				let mut buf = vec![0u8; 1024];
				// Spawning peer process
				let _ = tokio::spawn(async move {
					// Processing peer
					if let Err(e) = process(&mut stream, addr, &mut buf).await {
						println!("Error when processing TCP connection: {:?}", e);
					}
				});
			}
		});

		Ok(handle)
	}
}

async fn process(
	stream: &mut TcpStream,
	addr: SocketAddr,
	buf: &mut Vec<u8>
) -> Result<(), Box<dyn Error>> {
	Ok(())
}
