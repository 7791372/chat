use std::io::{self, Write};

pub fn draw() {
        // Move cursor to top of screen and print header
        print!("\x1B[1;1H\x1B[7mkonata@chat\x1B[0m");
        io::stdout().flush().unwrap();
}