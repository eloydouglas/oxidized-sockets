use std::{io, net::TcpStream};
use io::{Read, Write};

use crate::utils::parse_message;

pub fn start_client() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    println!("Escreva uma mensagem (ou exit para sair):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        
        if input == "exit" {
            println!("Closing connection");
            break;
        }

        stream.write_all(input.as_bytes()).unwrap();
        stream.flush().unwrap();

        handle_message(&stream);
    }
}

fn handle_message(mut stream: &TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if let Some(message) = parse_message(buffer, bytes_read) {
        println!("Server: {message}");
    }

}