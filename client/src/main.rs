mod core;

use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use self::core::server::Server;
use self::core::commands::command_loop;

fn main() {
    init();
}

// Initialize
fn init() {	
    let server = Server::default();
    let server_mutex =  Arc::new(Mutex::new(server));

    let command_thread = thread::spawn(move || {
        command_loop(server_mutex);
    });

    command_thread.join();
}
