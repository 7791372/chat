use std::io::{self, Write};
use crossterm::terminal::size;

pub fn draw() {
        let (width, _height) = size().unwrap();

        // Move cursor to top of screen and print header
        print!("\x1B[1;{}Hkonata@chat\x1B[0m", (width - 5) / 2);
        io::stdout().flush().unwrap();
}