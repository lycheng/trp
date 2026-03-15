use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");

    println!("TCP server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection from: {}", stream.peer_addr().unwrap());

                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let request = String::from_utf8_lossy(&buffer[..size]);
                        println!("Received: {}", request.trim());

                        let response = "Hello from TCP server!\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
