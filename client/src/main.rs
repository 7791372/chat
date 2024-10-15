// use tokio::net::TcpStream;

mod read_message;
mod write_message;
mod ui;

#[tokio::main]
async fn main() {
    // clear the entire screen
    print!("\x1B[2J\x1B[3J\x1B[H");

    // TODO: add port option in .config/chat
    // let stream = TcpStream::connect("127.0.0.1:11945").await.expect("Failed to connect");
    // let (reader, writer) = stream.into_split();

    ui::header::draw();
    ui::body::draw();
    ui::footer::draw();

    // let read_task = tokio::spawn(async move {
    //     read_message::read_message(reader).await;
    // });

    // let write_task = tokio::spawn(async move {
    //     write_message::write_message(writer).await;
    // });

    // let _ = tokio::join!(read_task, write_task);
}