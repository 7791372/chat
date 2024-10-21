// Handles all key presses done by the user

use tokio::net::tcp::OwnedWriteHalf;
use crossterm::event::{self, KeyCode, KeyEvent};
use tokio::sync::Mutex;
use std::sync::Arc;
use crossterm::terminal::{self};

use crate::util::update_input_field;
use crate::write_message::write_message; 

pub async fn input_handler(writer: OwnedWriteHalf, shared_variable: Arc<Mutex<String>>) {
    let mut writer = writer;

    loop {
        let mut input_lock = shared_variable.lock().await;

        if !event::poll(std::time::Duration::from_millis(100)).unwrap() {
            continue;
        }

        if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Enter => {
                    let input = input_lock.clone();
                    write_message(&mut writer, input).await;
                    input_lock.clear();
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    break;
                }
                KeyCode::Backspace => {
                    input_lock.pop();
                }
                KeyCode::Char(c) => {
                    input_lock.push(c);
                }
                _ => {}
            }
        }

        update_input_field(input_lock.to_string());
    }
}
