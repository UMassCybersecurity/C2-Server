use std::net::Ipv4Addr;
use std::collections::HashMap;

// Returns Map<&tr, Ipv4Addr> of white-listed IPs
// Gets white-listed IPs from Json file
pub fn get_white_listed_ips<'a>() -> HashMap<&'a str, Ipv4Addr> {
	let map: HashMap<&'a str, &'a str> = json_to_map("/config/white-listed-ips.json");
	let mut map_with_ip: HashMap<&'a str, Ipv4Addr> = HashMap::new();
	// TODO: Convert &'a str representation of IP address
	// To Ipv4Addr
	return map_with_ip;
}

// Adds/removes user(s) to white-listted IPs
// Adds/removes to Json file
// Returns false if unable to update list
pub fn update_listed_ips<'a>(ips: HashMap<&'a str, Ipv4Addr>) -> bool {
	// TODO: Update white-listed IPs and
	// user associated to IP from file
	// /config/white-listed-ips.json
	// call map_to_json to update
	return false;
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
