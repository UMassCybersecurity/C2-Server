#[path = "core/net/listener.rs"]
mod listener;

// Main function
fn main() {
	println!("Hello, world!");
	init();
}

// Initialize and start C2 server
fn init() {
	listener::init_host_listener(8080);
}
