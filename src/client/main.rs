use std::{
    io::{Read, Write},
    net::TcpStream,
};
fn main() {
    println!("Starting client...");
    let mut tcp_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // Successful Request
    tcp_stream.write_all(b"SHAKEHAND / HandShaker/0.1\r\n").unwrap();
    // Unsuccessful Request
    // tcp_stream.write_all(b"SHAKEHAND /foo HandShaker/0.1\r\n").unwrap();
    tcp_stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let _size = tcp_stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    println!("I got this message from server: {}", message);
}
