use std::net::TcpStream;
use std::io::{stdin, stdout, Write};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use super::client::Client;

#[path = "net/connection.rs"]
mod connection;


// Starts loops for commands
pub fn command_loop(client_mutex: Arc<Mutex<Client>>) {
	let mut input = String::new();
	loop {	
		print!(">>");
		stdout().flush();
		stdin().read_line(&mut input);			
		parse_command(input.trim(), Arc::clone(&client_mutex));	
		input.clear();	
	}
}

// Check to see if is command, if so execute
fn parse_command(cmd: &str, client_mutex: Arc<Mutex<Client>>) {
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
			let mut client = client_mutex.lock().unwrap();
			client.set_server(socket);
		}
		_ => return
	}
}
