use std::net::Ipv4Addr;
use std::collections::HashMap;

use std::io;

// TODO: https://trello.com/c/v7gVKowM
pub fn get_white_listed_ips<'a, 'b>() -> HashMap<&'a str, &'a Ipv4Addr> {
	let map: HashMap<&'b str, &'b str> = json_to_map("/config/white-listed-ips.json");
	let mut map_with_ip: HashMap<&'a str, &'a Ipv4Addr> = HashMap::new();
	return map_with_ip;
}

// TODO: https://trello.com/c/p3KmSmTw
pub fn add_white_listed_ip(user: &str, ip: &Ipv4Addr) -> bool {
	return false;
}

//TODO: https://trello.com/c/bGB8On9n
pub fn remove_white_listed_ip(user: &str) -> bool {
	return false;
}

//TODO: https://trello.com/c/vSm6iisi
fn update_white_listed_ips(ips: HashMap<&str, &str>) -> bool {
	return map_to_json("/config/white-listed-ips.json", ips);
}

// TODO: https://trello.com/c/xFCwAs9x
fn json_to_map(path: &str) -> HashMap<&str, &str> {
	let map: HashMap<&str, &str> = HashMap::new();
	return map;
}

// TODO: https://trello.com/c/5gNSfzY9
fn map_to_json(path: &str, map: HashMap<&str, &str>) -> bool {
	return false;
}
