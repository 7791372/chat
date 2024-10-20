use tokio::net::TcpStream;
use crossterm::terminal;

mod read_message;
mod write_message;
mod input_handler;
mod ui;

#[tokio::main]
async fn main() { 
    // TODO: add port option in .config/chat
    let stream = TcpStream::connect("127.0.0.1:11942").await.expect("Failed to connect");
    let (reader, writer) = stream.into_split();

    terminal::enable_raw_mode().unwrap();
    print!("\x1B[2J\x1B[3J\x1B[H");

    ui::header::draw();
    ui::footer::draw();

    let _read_task = tokio::spawn(async move {
        read_message::read_message(reader).await;
    });

    let write_task = tokio::spawn(async move {
        input_handler::handle_input(writer).await;
    });

    let _ = tokio::join!(write_task);
}
