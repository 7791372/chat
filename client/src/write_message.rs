// Writes a message to the server

use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

pub async fn write_message(writer: &mut OwnedWriteHalf, message: String) {
    let name = "anon";
    let formatted_message = format!("{}:{}", name, message);

    if let Err(e) = writer.write_all(formatted_message.as_bytes()).await {
        eprintln!("Failed to write message: {}", e); 
    }
}
