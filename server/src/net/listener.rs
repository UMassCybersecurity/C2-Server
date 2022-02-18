use std::thread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io;

#[path = "../config.rs"]
mod config;

#[path = "../log.rs"]
mod log;

// Initializes multi-threaded listener for server
// Note: potentially handle port as &str
// TODO: Add return for Result<bool, io::Error>
pub fn init_host_listener(port: u16) {
	// Get socket
	let socket = create_socket(port);
	// Start listener
	let listener = TcpListener::bind(socket).unwrap();
	// Handle connections
	// TODO: Create thread to handle connections so program execution
	// can continue
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		thread::spawn(|| {
			handle_client_connection(stream);
		});
	}
}

// Handles connections from client application
fn handle_client_connection(connection: TcpStream) {
	// TODO: Filter IPs with function is_allowed_ip
	// and log all connections (both attempted and
	// established)
	log::log("", connection);
}

// Activates connection to C2 server with client
fn activate_connection(connection: TcpStream) {
	// TODO: Create connection and allow commands to be sent and received
	log::log("", connection);
}

// Checks provided ip with white-listed IPs
fn is_allowed_ip(ip: Ipv4Addr) -> bool {
	// TODO: Check ip with list of IPs that are
	// whitelisted via get_white_listed_ip_addr
	return false;
}

// Creates a new IPv4 socket for listeners
fn create_socket(port: u16) -> SocketAddr {
	let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
	return socket;
}
