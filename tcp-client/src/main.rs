use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write(b"Hello, server!").unwrap();
    stream.flush().unwrap();
}
