use std::net::{IpAddr, TcpStream};

// Type for Session struct
enum SessionType {
    BIND,
    REVERSE,
}

// Session struct
struct Session {
    connection: TcpStream,
    session_type: SessionType,
}

// TODO: Start session from TcpStream
pub fn start_session(connection: TcpStream) {}

// TODO: Create session
fn create_session(connection: TcpStream) -> Session {
    return Session {
        connection: connection,
        session_type: SessionType::BIND,
    };
}
