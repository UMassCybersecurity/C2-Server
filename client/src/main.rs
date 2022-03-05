#[path = "core/commands.rs"]
mod commands;

use std::env;

fn main() {
    init();
}

// Initialize
fn init() {	
	commands::command_loop();
}
