// Reads a message from the server

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use std::io::{self, Write};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::util::update_input_field;

pub async fn read_message(reader: OwnedReadHalf, shared_variable: Arc<Mutex<String>>) {
    let mut buf_reader = BufReader::new(reader);
    let mut buffer = vec![0; 512];

    while let Ok(n) = buf_reader.read(&mut buffer).await {
        if n == 0 {
            eprintln!("Connection closed");
            break;
         }

        //  TODO: this is pretty ugly ansi code all over the place so somehow make it look nice later
        let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        if let Some((name, content)) = parse_message(&msg) {
            print!("\x1b[s");
            println!("\x1B[2K\r{}: {}", name, content);

            let input_lock = shared_variable.lock().await;
            update_input_field(input_lock.to_string());

            print!("\x1b[u");
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