use std::net::TcpStream;

#[derive(Default)]
pub struct Client {
    sever_connetion: Option<TcpStream>,
}

impl Client {
    pub fn set_server(&mut self, socket: TcpStream) {
        self.sever_connetion = Some(socket);
    }
}