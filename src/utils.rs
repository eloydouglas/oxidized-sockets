use std::fs::File;
// use std::fs::File;
use std::{net::TcpStream, str};
use std::io::{self, Write, Read};

pub fn parse_messages(buffer: [u8; 512], bytes_read: usize) -> Option<Vec<String>> {
    if bytes_read > 0 {
        let raw_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        let messages: Vec<String> = raw_data.split("::END::")
            .filter(|msg| !msg.trim().is_empty())
            .map(|msg| msg.trim().to_string()) 
            .collect();

        return Some(messages);
    }

    return None;
}

pub fn send_message(mut stream: &TcpStream, message: &str) {
    let message = format!("{}::END::", message);
    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn handle_file(mut stream: &TcpStream, curr_dir: &str, filename: String) {
    println!("Receivig file...");

    let mut file = File::create(curr_dir.to_owned() + "/" + &filename).unwrap();

    // Buffer maior para minimizar os writes:
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        // Preciso saber quando o file terminou, então parseio os últimos 12 bytes para ver
        // Se recebi a mensagem indicando o fim do arquivo
        if bytes_read >= 12 && &buffer[bytes_read - 12..bytes_read] == b"::END_FILE::" {
            //Salva os últimos bytes do arquivo
            file.write_all(&buffer[..bytes_read - 12]).unwrap();
            break;
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
    }
    println!("File received!");
}

pub fn send_file(stream: &mut TcpStream, filepath: String) {
    println!("Sending file...");
    if let Ok(mut file) = File::open(&filepath) {
        let mut buffer = [0; 1024];
        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            stream.write_all(&buffer[..bytes_read]).unwrap();
        }
        stream.write_all(b"::END_FILE::").unwrap();
    } else {
        println!("An error occurred while trying to upload the file.")
    };

    println!("File sent!");
}

// pub fn receive_file(stream: &mut TcpStream, curr_dir: &String, filename: &str) {
//     let mut buffer = [0; 1024];
//     let mut file = File::create(format!("{}/{}", curr_dir, filename))
//         .expect("Failed to create file");

//     let mut size_buffer = [0; 8];
//     if let Err(e) = stream.read_exact(&mut size_buffer) {
//         eprintln!("Error reading file size: {}", e);
//         return;
//     }
    
//     let file_size = u64::from_le_bytes(size_buffer);
//     println!("Receiving file of size: {}", file_size);

//     let mut received = 0;
//     while received < file_size {
//         match stream.read(&mut buffer) {
//             Ok(0) => break,
//             Ok(n) => {
//                 file.write_all(&buffer[..n]).expect("Error writing to file");
//                 received += n as u64;
//             },
//             Err(e) => {
//                 eprintln!("Error receiving file data: {}", e);
//                 break;
//             }
//         }
//     }

//     println!("File received successfully");
// }

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}