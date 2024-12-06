use std::{io::Read, net::{TcpListener, TcpStream}};

use crate::{
    server::commands::{
        ChangeDir, Help, ListDir, MakeDir, RemovePath
    },
    utils::{handle_file, parse_messages, send_file, send_message}
};

pub fn start(root_dir: Option<String>, port: Option<i16>) {
    let root_dir = root_dir.unwrap_or(String::from("./"));
    let port = port.unwrap_or(7878);

    let addr = "127.0.0.1:".to_owned() + &port.to_string(); 

    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listenning to connections in port {port}");
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), &root_dir);
    }
}

fn handle_connection(mut stream: TcpStream, root_dir: &String) {
    let mut buffer = [0; 512];

    println!("Connection received");

    let mut curr_dir = root_dir.clone();
    let message = "::CURR_DIR::".to_owned() + &curr_dir;
    send_message(&stream, &message);
    let messages_list = Help::call();
    for message in messages_list {
        let message = "::DISPLAY::".to_owned() + &message;
        send_message(&stream, &message);
    }
    send_message(&stream, "::READY::");

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => { // Conexão fechada pelo cliente
                println!("Client closed connection");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        };

        if let Some(messages) = parse_messages(buffer, bytes_read) {
            for message in messages {
                println!("Received: {message}");

                match message.as_str() {
                    command if command.contains("cd") => {
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if let Some(path) = parts.get(1) {
                            curr_dir = ChangeDir::call(&curr_dir, path.to_string());
                        } else {
                            curr_dir = ChangeDir::call(&curr_dir, "".to_owned());
                        }
                    },
                    command if command.contains("mkdir") => {
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if let Some(path) = parts.get(1) {
                            MakeDir::call(&curr_dir, path.to_string());
                        }
                    },
                    command if command.contains("rm") => {
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if let Some(path) = parts.get(1) {
                            if let Err(_) = RemovePath::call(&curr_dir, path.to_string()) {
                                send_message(&stream, "Something went wrong while trying to delete the path");
                            };
                        }
                    },
                    command if command.starts_with("upload") => {
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if let Some(filename) = parts.get(1) {
                            handle_file(&mut stream, &curr_dir, filename.to_string());
                        }
                    },
                    command if command.starts_with("download") => {
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if let Some(filename) = parts.get(1) {
                            send_message(&stream, &("::UPLOAD::".to_owned() + filename.to_owned()));
                            send_file(&mut stream, String::from(curr_dir.clone() + "/" + filename));
                        }
                    },
                    "help" => {
                        let messages_list = Help::call();

                        for message in messages_list {
                            let message = "::DISPLAY::".to_owned() + &message;
                            send_message(&stream, &message);
                        }
                    },
                    "ls" => {
                        let paths_list = ListDir::call(&curr_dir);

                        for path in paths_list {
                            let message = "::DISPLAY::".to_owned() + &path;
                            send_message(&stream, &message);
                        }
                    },
                    "" => (),
                    _  => {
                        send_message(&stream, "::DISPLAY::Invalid command!");
                        let messages_list = Help::call();

                        for message in messages_list {
                            let message = "::DISPLAY::".to_owned() + &message;
                            send_message(&stream, &message);
                        }
                    }
                    
                }

            }

            let message = "::CURR_DIR::".to_owned() + &curr_dir;
            send_message(&stream, &message);
            send_message(&stream, "::READY::");
        }
    }

}