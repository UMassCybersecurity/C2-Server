mod core;

use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use self::core::client::Client;
use self::core::commands::command_loop;

fn main() {
    init();
}

// Initialize
fn init() {	
    let client = Client::default();
    let client_mutex =  Arc::new(Mutex::new(client));

    let command_thread = thread::spawn(move || {
        command_loop(client_mutex);
    });

    command_thread.join();
}
