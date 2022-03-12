use std::net::TcpStream;
use std::io::{BufReader};

#[derive(Default)]
pub struct Client {
    client_connection: Option<TcpStream>,
}

impl Client {
    pub fn set_client(&mut self, socket: TcpStream) {
        self.client_connection = Some(socket);
    }

    pub fn get_reader(&mut self) -> BufReader<&TcpStream> {
        return BufReader::new(self.client_connection.as_ref().unwrap());
    }
}