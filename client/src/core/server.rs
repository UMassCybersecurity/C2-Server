use std::net::TcpStream;
use std::io::Write;
#[derive(Default)]

pub struct Server {
    sever_connection: Option<TcpStream>,
}

impl Server {
    pub fn set_server(&mut self, socket: TcpStream) {
        self.sever_connection = Some(socket);
    }

    pub fn send(&mut self, data: &str) {
        self.sever_connection.as_ref().unwrap().write(data.as_bytes());
    }

    pub fn is_connected(&mut self) -> bool {
        !self.sever_connection.is_none()
    }
}