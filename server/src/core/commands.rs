use std::net::TcpStream;
use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod clients;

use super::clients::Client;

// Starts loops for commands
pub fn command_loop(mut client_: Client) {
	let mut reader = client_.get_reader();
	loop {	
		let mut input = String::new();
		reader.read_line(&mut input);
		println!("{:?}", input);
		parse_command(input.trim());
	}
}

// Check to see if is command, if so execute
fn parse_command(cmd: &str) {
	let tokens: Vec<&str> = cmd.split_whitespace().collect::<Vec<&str>>();
	if tokens.len() == 0 {
		return
	}	
	match tokens[0] {
		"help" => {
			println!("TODO Implement help")
		}
		_ => {
			println!("{:?}", tokens);
		}
	}
}
