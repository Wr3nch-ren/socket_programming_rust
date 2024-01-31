use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Starting server for recieve 1 request...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(1) {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("Received a message, closing...")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _size = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    println!("I got this message from client: {}", message);

    let shakehand = "SHAKEHAND / HandShaker/0.1\r\n";

    let (ptc_ver, status_code, status_phrase) = if buffer
    .starts_with(shakehand.as_bytes()) {
        ("HandShaker/0.1", "3663", "HANDSHAKE SUCCESS")
    } else {
        ("HandShaker/0.1", "37767", "HANDSHAKE FAILED")
    };

    let response = format!(
        "PTC_VER: {}\r\nSTATUS_CODE: {}\r\nSTATUS_PHRASE: {}\r\n\r\n",
        ptc_ver, status_code, status_phrase
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}