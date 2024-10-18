// Handles all key presses done by the user

use tokio::net::tcp::OwnedWriteHalf;
use crossterm::event::{self, KeyCode, KeyEvent};
use std::io::{self, Write};

use crate::write_message::write_message;

pub async fn handle_input(writer: OwnedWriteHalf) {
    let mut input = String::new();
    let mut writer = writer;

    loop {
        if !event::poll(std::time::Duration::from_millis(100)).unwrap() {
            continue;
        }

        if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Enter => {
                    // TODO: this single line somehow prevents double printing pls fix later
                    print!("\r");
                    write_message(&mut writer, input.clone()).await;
                    input.clear();
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Backspace => {
                    input.pop();
                    print!("\r{}", input);
                    io::stdout().flush().unwrap();
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("\r{}", input);
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }
    }
}
