use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use sea_orm::DatabaseConnection;

use tokio::net::{TcpStream, TcpListener, UdpSocket};
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
	/// Connection buffer
	pub buffer: Arc<Mutex<Vec<u8>>>,
	/// Connections to friends
	pub friends: Arc<Mutex<HashMap<String, Option<TcpStream>>>>,
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
		let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

		Server {
			config: config.clone(),
			address,
			database,
			runtime,
			buffer: Arc::new(Mutex::new(vec![0; 1024])),
			friends: Arc::new(Mutex::new(HashMap::new()))
		}
	}

	/// Run server
	pub async fn run(&self) {
		let mut handles = vec![];

		if self.config.server.listen.tcp {
			let handle = self.start_tcp_server().await;
			handles.push(
				handle.unwrap_or_else(
					|e|
						panic!("Failed to start TCP listener on {}: {:?}", self.address, e)
				)
			);
		}

		if self.config.server.listen.udp {
			let handle = self.start_udp_server().await;
			handles.push(
				handle.unwrap_or_else(
					|e|
						panic!("Failed to start UDP listener on {}: {:?}", self.address, e)
				)
			);
		}
		// Looping almost forever, because listeners run until error, I guess
		futures::future::join_all(handles).await;
	}

	async fn start_tcp_server(&self) -> Result<JoinHandle<()>, Box<dyn Error>> {
		// Creating TCP listener
		let socket = TcpListener::bind(self.address).await?;
		let buffer = Arc::clone(&self.buffer);
		// Creating TCP listener thread
		Ok(self.runtime.spawn(async move {
			loop {
				let mut buf = buffer.lock().await;
				let (stream, addr) = socket.accept().await.unwrap();
				println!("Accepted connection: {:?} - {:?}", addr, stream);
				if let Err(e) = process_tcp(stream, addr, &mut buf.to_vec()).await {
					println!("Error when processing TCP connection: {:?}", e);
				}
			}
		}))
	}

	async fn start_udp_server(&self) -> Result<JoinHandle<()>, Box<dyn Error>> {
		// Creating UDP socket
		let socket = UdpSocket::bind(self.address).await?;
		let buffer = Arc::clone(&self.buffer);
		Ok(self.runtime.spawn(async move {
			loop {
				let mut buf = buffer.lock().await;
				let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
				println!("Received {:?} bytes from {:?}", len, addr);
				if let Err(e) = process_udp(&socket, addr, &mut buf.to_vec(), len).await {
					println!("Error when processing UDP connection: {:?}", e);
				}

			}
		}))
	}
}

async fn process_tcp(
	stream: TcpStream,
	addr: SocketAddr,
	buf: &mut Vec<u8>
) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn process_udp(
	socket: &UdpSocket,
	addr: SocketAddr,
	buf: &mut Vec<u8>,
	len: usize
) -> Result<(), Box<dyn Error>> {
	Ok(())
}
