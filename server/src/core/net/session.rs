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

pub fn start_session(connection: TcpStream) {}

// TODO: Create session
fn create_session(connection: TcpStream) -> Session {
    return Session {
        connection: connection,
        session_type: SessionType::BIND,
    };
}

// TODO: Transfer session to new ip
fn transfer_session(session: Session, ip: IpAddr, session_type: SessionType) -> Session {
    // TODO: Call create_bind_session or create_reverse_session based on session_type
    return Session {
        connection: session.connection,
        session_type: session_type,
    };
}

// TODO: Return TcpStream for bind connection
// fn create_bind_session() -> TcpStream {}

// TODO: Return TcpStream for reverse connection
// fn create_reverse_session() -> TcpStream {}
