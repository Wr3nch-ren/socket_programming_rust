use std::{
    io::{Read, Write},
    net::TcpStream,
};
pub fn main() {
    println!("Starting client...");
    let mut tcp_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    tcp_stream.write_all(b"Hello, Server!").unwrap();
    tcp_stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let _size = tcp_stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    println!("I got this message from server: {}", message);
}
