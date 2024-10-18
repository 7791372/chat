// Writes a message to the server

use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

pub async fn write_message(writer: &mut OwnedWriteHalf, message: String) {
    let _ = writer.write_all(message.as_bytes()).await;
}
