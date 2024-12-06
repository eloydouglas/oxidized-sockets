use std::fs::File;
// use std::fs::File;
use std::{net::TcpStream, str};
use std::io::{self, Write, Read};

const END_FILE_MARKER: &[u8; 12] = b"::END_FILE::";
const END_MESSAGE_MARKER: &str = "::END::";

pub fn parse_messages(buffer: [u8; 512], bytes_read: usize) -> Option<Vec<String>> {
    if bytes_read > 0 {
        let raw_data = String::from_utf8_lossy(&buffer[..bytes_read]);
        let messages: Vec<String> = raw_data.split(END_MESSAGE_MARKER)
            .filter(|msg| !msg.trim().is_empty())
            .map(|msg| msg.trim().to_string()) 
            .collect();

        return Some(messages);
    }

    return None;
}

pub fn send_message(mut stream: &TcpStream, message: &str) {
    let message = format!("{}{}", message, END_MESSAGE_MARKER);
    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn handle_file(mut stream: &TcpStream, curr_dir: &str, filename: String) {
    println!("Receivig file...");    
    let path = curr_dir.to_owned() + "/" + &filename;
    let mut file = File::create(path).unwrap();

    // Buffer maior para minimizar os writes:
    let mut temp_buffer = [0; 1024];
    
    // Buffer variável para poder contar com os marcadores de fim de arquivo
    let mut buffer = Vec::new();

    while let Ok(bytes_read) = stream.read(&mut temp_buffer) {
        if bytes_read == 0 {
            break;
        }

        // Adiciona os bytes lidos ao buffer dinâmico
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // Se encontrar o marcador, retorna a posição e salva no arquivo todos os bytes até o marcador
        if let Some(pos) = contains_end_file_marker(&buffer) {
            file.write_all(&buffer[..pos]).unwrap();
            break;
        }
    }
    // Gambi pra resetar o ready já que eu jogo fora os bytes que sobram
    send_message(&stream, "");
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
        stream.write_all(END_FILE_MARKER).unwrap();
    } else {
        println!("An error occurred while trying to upload the file.")
    };

    println!("File sent!");
}

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn contains_end_file_marker(buffer: &[u8]) -> Option<usize> {
    // Procura o marcador na janela de bytes do tamanho do marcador
    buffer
        .windows(END_FILE_MARKER.len()) 
        .position(|window| window == END_FILE_MARKER) 
}