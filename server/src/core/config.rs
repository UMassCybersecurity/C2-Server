use std::net::Ipv4Addr;
use std::collections::HashMap;

use std::io;

// Returns Map<&tr, Ipv4Addr> of white-listed IPs
// Gets white-listed IPs from Json file
pub fn get_white_listed_ips<'a>() -> HashMap<&'a str, &'a Ipv4Addr> {
	let map: HashMap<&'a str, &'a str> = json_to_map("/config/white-listed-ips.json");
	let mut map_with_ip: HashMap<&'a str, &'a Ipv4Addr> = HashMap::new();
	// TODO: Convert &'a str representation of IP address
	// To Ipv4Addr
	return map_with_ip;
}

// Add ip and user to white-listed-ips.json
// Return false if user already exists
pub fn add_white_listed_ip(user: &str, ip: &str) -> bool {
	// TODO: Add user and ip to IP white list
	return false
}

// Removes user and associated user from white-listed-ips.json
// Returns false if user does not exist
pub fn remove_white_listed_ip(user: &str) -> bool {
	// TODO: Remove user and associated IP from white-list
	return false
}

// Adds/removes user to white-listted IPs
fn update_white_listed_ips<'a>(ips: HashMap<&'a str, &'a Ipv4Addr>) {
	// TODO: Update white-listed IPs and user associated to IP from file /config/white-listed-ips.json call map_to_json to update
}

// Retrieves Json file from path and converts to serde_json Map
// Returns HashMap representation of json file
fn json_to_map<'a>(path: &str) -> HashMap<&'a str, &'a str> {
	let map: HashMap<&'a str, &'a str> = HashMap::new();
	// TODO: Read json file from path and return HashMap of it
	return map;
}

// Convert hashmap to json
fn map_to_json<'a>(path: &str, map: HashMap<&'a str, &'a str>) -> bool {
	// TODO: Update json file on path to map
	return false;
}
