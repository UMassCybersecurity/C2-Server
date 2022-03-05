use std::net::TcpStream;
use std::io::{stdin, stdout, Write};
use std::net::Ipv4Addr;
use std::str::FromStr;

// Starts loops for commands
pub fn command_loop() {
	let mut input = String::new();
	loop {	
		print!(">>");
		stdout().flush();
		stdin().read_line(&mut input);			
		parse_command(input.trim());	
		input.clear();	
	}
}

// Check to see if is command
fn parse_command(cmd: &str) {
	let tokens: Vec<&str> = cmd.split_whitespace().collect::<Vec<&str>>();
	if tokens.len() == 0 {
		return
	}	
	match tokens[0] {
		"help" => {
			println!("TODO Implement help")
		}
		"connect" => {
			//connection::init_connection(Ipv4Addr::from_str(tokens[1]), tokens[2].parse::<u16>().unwrap());
		}
		_ => return
	}
}
