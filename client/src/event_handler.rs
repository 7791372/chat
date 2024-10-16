// Handles all key presses done by the user

use std::io::{self, Write};
use crossterm::{
    event::{self, KeyCode, KeyEvent, read},
    terminal::{self},
};
use tokio::net::tcp::OwnedWriteHalf;

use crate::write_message::write_message;

pub async fn handle_events(writer: OwnedWriteHalf) {
    let mut input = String::new();

    loop {
        if !event::poll(std::time::Duration::from_millis(500)).unwrap() {
            continue;
        }

        if let event::Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            handle_keypress(code, writer, &mut input).await;
        }
    }
}

async fn handle_keypress(code: KeyCode, writer: OwnedWriteHalf, input: &mut String) {
    match code {
        KeyCode::Enter => write_message(writer, input).await,
        KeyCode::Esc => handle_esc(),
        KeyCode::Backspace => handle_backspace(input),
        KeyCode::Char(c) => handle_char(c, input),
        _ => {}
    }
}

fn handle_esc() {

}

fn handle_backspace(input: &mut String) {
    input.pop();
    print!("\r{}", input);
    io::stdout().flush().unwrap();
}

fn handle_char(c: char, input: &mut String) {
    input.push(c);
    print!("\r{}", input);
    io::stdout().flush().unwrap();
}
