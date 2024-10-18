use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:11945").await.unwrap();
    let (tx, _) = broadcast::channel(16);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut reader, mut writer) = socket.into_split();

        tokio::spawn(async move {
            let mut buffer = vec![0; 512];
            while let Ok(n) = reader.read(&mut buffer).await {
                if n == 0 { break; }

                let msg = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                if tx.send(msg).is_err() {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if writer.write_all(msg.as_bytes()).await.is_err() {
                    break;
                }
                if writer.write_all(b"\n").await.is_err() {
                    break;
                }
            }
        });
    }
}
