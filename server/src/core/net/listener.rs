use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::thread;

#[path = "../config.rs"]
mod config;

#[path = "../log.rs"]
mod log;

#[path = "../commands.rs"]
mod commands;

#[path = "../clients.rs"]
mod clients;

#[path = "encryption.rs"]
mod encryption;

#[path = "session.rs"]
mod session;

// Starts listened loop
pub fn init_host_listener(port: u16) {
	let listener = TcpListener::bind(create_socket(port)).unwrap();
	// Loop to handle connections
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		thread::spawn(|| {
			handle_client_connection(stream);
		});
	}
}

// Handle all connections received
fn handle_client_connection(connection: TcpStream) {
	// Log connection attempts
	log::log(
		format!(
			"{:?} is attempting to connect",
			connection.peer_addr().unwrap().ip()
		)
		.as_str(),
	);
	// Accept or reject connection
	match is_allowed_ip(connection.peer_addr().unwrap().ip()) {
		true => {
				log::log(format!("{:?} is allowed to connect", connection
					.peer_addr()
					.unwrap()
					.ip())
					.as_str());
				activate_connection(connection);
		}
		false => {
				log::log(format!("{:?} is not allowed to connect", connection
					.peer_addr()
					.unwrap()
					.ip())
					.as_str()); 
				connection.shutdown(Shutdown::Both);
		}
	}
}	

// TODO: https://trello.com/c/p8NmLJxO
fn activate_connection(connection: TcpStream) {
	let mut client = clients::Client::default();
	client.set_client(connection);

	thread::spawn(|| {
		commands::command_loop(client);
	});
}

// TODO: https://trello.com/c/Q9sxhqQH
fn is_allowed_ip(ip: IpAddr) -> bool {
	return true;
}

// Returns SocketAddr for a new listener connection
fn create_socket(port: u16) -> SocketAddr {
	return SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
}
