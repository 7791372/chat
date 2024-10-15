use std::io::{self, Write};
use crossterm::terminal::size;

pub fn draw() {
    let (_width, height) = size().unwrap();

    // Draw input prompt at the bottom
    print!("\x1B[{};1H\x1B[2mkonata is typing\x1B[0m", height);
    print!("\x1B[{};1H$", height-1);
    io::stdout().flush().unwrap();
}