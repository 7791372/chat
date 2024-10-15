use std::io::{self, Write};
use crossterm::terminal::size;

pub fn draw() {
    let (width, height) = size().unwrap();

    // draw the chat lines
    if height > 2 {
        for row in 3..height-2 {
            print!("\x1B[{};1H~", row);
        }
    }

    // draw the sidebar if terminal is wide enough
    if width > 69 {
        for row in 1..height+1 {
            print!("\x1B[{};{}H┃", row, width-21);
        }
    }

    // tests
    print!("\x1B[10;1H250115 00:00");
    print!("\x1B[11;1H[yuna@oldinternet] xd");

    io::stdout().flush().unwrap();
}