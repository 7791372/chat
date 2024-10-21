use crossterm::terminal::size;
use std::io::{self, Write};

pub fn update_input_field(input_lock: String) {
    let (width, _height) = size().unwrap();

    let visible_input = if input_lock.len() > width as usize {
        input_lock[input_lock.len() - width as usize..].to_string()
    } else {
        input_lock.to_string()
    };

    print!("\x1B[2K\r{}", visible_input);
    io::stdout().flush().unwrap();
}