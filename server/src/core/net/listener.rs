use std::thread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io;

#[path = "../config.rs"]
mod config;

#[path = "../log.rs"]
mod log;

#[path = "encryption.rs"]
mod encryption;

// TODO: https://trello.com/c/FftuRBn1
pub fn init_host_listener(port: u16) -> Result<bool, io::Error> {
	let socket = create_socket(port);
	let listener = TcpListener::bind(socket).unwrap();
	// Loop to handle connections
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		thread::spawn(|| {
			handle_client_connection(stream);
		});
	}
	return Ok(true)
}

// TODO: https://trello.com/c/Tk2AFl04
fn handle_client_connection(connection: TcpStream) {
	log::log(format!("{:?} is attempting to connect", connection.peer_addr().unwrap().ip()).as_str());
	
	match is_allowed_ip(connection.peer_addr().unwrap().ip()) {
		true => {
				log::log(format!("{:?} is allowed", connection
					.peer_addr()
					.unwrap()
					.ip())
					.as_str());
				activate_connection(connection);
		}
		false => {
				log::log(format!("{:?} is not allowed", connection
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
	log::log("");
}

// TODO: https://trello.com/c/Q9sxhqQH
fn is_allowed_ip(ip: IpAddr) -> bool {
	return true;
}

// TODO: https://trello.com/c/XWKCgSqY
fn create_socket(port: u16) -> SocketAddr {
	let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
	return socket;
}
