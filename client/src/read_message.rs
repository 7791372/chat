// Reads a message from the server

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use std::io::{self, Write};

pub async fn read_message(reader: OwnedReadHalf) {
    let mut buf_reader = BufReader::new(reader);
    let mut buffer = vec![0; 512];

    while let Ok(n) = buf_reader.read(&mut buffer).await {
        if n == 0 {
            eprintln!("Connection closed");
            break;
         }

        let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        if let Some((name, content)) = parse_message(&msg) {
            print!("\r\x1B[2K{}: {}", name, content);
            io::stdout().flush().unwrap();
        }
    }
}

fn parse_message(msg: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = msg.splitn(2, ':').collect();
    if parts.len() == 2 {
        return Some((parts[0], parts[1]));
    }
    None
}