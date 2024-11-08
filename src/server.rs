use std::{io::Read, net::{TcpListener, TcpStream}};

use crate::utils::parse_message;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server started and waiting for connections");
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    println!("Connection received");

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => { // Conexão fechada pelo cliente
                print!("Client closed connection");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        };

        if let Some(message) = parse_message(buffer, bytes_read) {
            println!("Client: {message}");
        }
    }

}