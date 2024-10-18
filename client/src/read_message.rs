// Reads a message from the server

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

pub async fn read_message(reader: OwnedReadHalf) {
    let mut buf_reader = BufReader::new(reader);
    let mut buffer = vec![0; 512];

    while let Ok(n) = buf_reader.read(&mut buffer).await {
        // server probably closed if this is true
        if n == 0 { break };

        let msg = String::from_utf8_lossy(&buffer[..n]);
        print!("{}", msg)
    }
}
