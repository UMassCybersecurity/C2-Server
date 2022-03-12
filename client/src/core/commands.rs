use std::net::TcpStream;
use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use super::server::Server;

#[path = "net/connection.rs"]
mod connection;


// Starts loops for commands
pub fn command_loop(server_mutex: Arc<Mutex<Server>>) {
	let mut reader = BufReader::new(stdin());
	loop {	
		print!(">> ");
		stdout().flush();
		let mut input = String::new();
		reader.read_line(&mut input);			
		parse_command(&input, Arc::clone(&server_mutex));	
		input.clear();	
	}
}

// Check to see if is command, if so execute
fn parse_command(cmd: &str, server_mutex: Arc<Mutex<Server>>) {
	let tokens: Vec<&str> = cmd.split_whitespace().collect::<Vec<&str>>();
	if tokens.len() == 0 {
		return
	}	
	match tokens[0] {
		"help" => {
			println!("TODO Implement help")
		}
		"connect" => {
			let socket = connection::init_connection(IpAddr::from_str(tokens[1]).unwrap(), tokens[2].parse::<u16>().unwrap());
			let mut server = server_mutex.lock().unwrap();
			server.set_server(socket);
		}
		_ => {
			let mut server = server_mutex.lock().unwrap();
			if server.is_connected() {
				server.send(cmd)
			}
		}
	}
}
