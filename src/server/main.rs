use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub fn main() {
    println!("Starting server for recieve 1 request...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(1) {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let _size = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("I got this message from client: {}", message);
        stream.write_all(b"Hello, Client!").unwrap();
        stream.flush().unwrap();
    }

    println!("Received a message, closing...")
}
