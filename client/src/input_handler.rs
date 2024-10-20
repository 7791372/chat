// Handles all key presses done by the user

use tokio::net::tcp::OwnedWriteHalf;
use crossterm::event::{self, KeyCode, KeyEvent};
use std::io::{self, Write};
use crossterm::terminal::{self};

use crate::write_message::write_message; 

pub async fn input_handler(writer: OwnedWriteHalf) {
    let mut input = String::new();
    let mut writer = writer;

    loop {
        if !event::poll(std::time::Duration::from_millis(100)).unwrap() {
            continue;
        }

        if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Enter => {
                    write_message(&mut writer, input.clone()).await;
                    input.clear();
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    break;
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                _ => {}
            }
        }
        print!("\x1B[2K\r{}", input);
        io::stdout().flush().unwrap();
    }
}
