use std::{fs, io, net::TcpStream, path::Path};
use io::{Read, Write};

use crate::utils::{handle_file, parse_messages, read_input, send_file, send_message};

pub fn start(addr: Option<String>) {
    let addr = addr.unwrap_or("127.0.0.1:7878".to_owned());

    let mut stream = TcpStream::connect(addr).unwrap();
    let mut curr_dir = String::from("");

    println!("\nConnected!\n");

    loop {
        'listener: loop {
            if let Some(messages ) = handle_messages(&mut stream) {
                for message in messages {
                    match message.as_str() {
                        dir if dir.contains("::CURR_DIR::") =>
                            curr_dir = message.strip_prefix("::CURR_DIR::").unwrap().to_string(),
                        display if display.contains("::DISPLAY::") =>
                            println!("{}", message.strip_prefix("::DISPLAY::").unwrap().to_string()),
                        upload if upload.contains("::UPLOAD::") => {
                            let filename = message.strip_prefix("::UPLOAD::").unwrap().to_string();
                            handle_receive_file(&stream, filename);
                        },
                        "::READY::" => {
                            println!();
                            break 'listener
                        },
                        _ => ()
                    }
                }
            }
        }

        print!("{curr_dir} > ");
        std::io::stdout().flush().unwrap();

        let input = read_input();
        println!();

        match input.as_str() {
            "exit" => {
                println!("Closing connection");
                break;
            },
            command if command.contains("upload") => {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if let Some(filepath) = parts.get(1) {
                    let filename = filepath.split("/").last().unwrap();
                    send_message(&stream, &("upload ".to_owned() + &filename));
                    send_file(&mut stream, filepath.to_string());
                }
            },
            _ => send_message(&stream, &input)
        }
    }
}

fn handle_messages(mut stream: &TcpStream) -> Option<Vec<String>> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    parse_messages(buffer, bytes_read)
}

fn handle_receive_file(stream: &TcpStream, filename: String) {
    let server_files = "server_files";

    if !Path::new(server_files).is_dir() {
        fs::create_dir_all(server_files).unwrap();
    }
    
    handle_file(&stream, server_files, filename);
}