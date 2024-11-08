use std::str;

pub fn parse_message(buffer: [u8; 512], bytes_read: usize) -> Option<String> {
    if bytes_read > 0 {
        let message = str::from_utf8(&buffer[..bytes_read]).unwrap();
        return Some(message.to_string());
    }

    return None;
}