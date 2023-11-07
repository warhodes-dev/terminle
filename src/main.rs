#![allow(dead_code)]

use std::io;

use crossterm::{
    cursor, 
    style, 
    terminal::{self, disable_raw_mode, ClearType}, 
    event::{self, KeyEvent, KeyCode, KeyModifiers, Event},
    ExecutableCommand, 
};
use anyhow::{Result, anyhow};

mod config;
use config::Config;

mod wordle;
use wordle::{Wordle, Hint};

fn main() {
    let config = Config::parse();
    
    let wordle = Wordle::new(config.challenge_number);

    if let Err(e) = app(wordle) {
        terminal::disable_raw_mode()
            .expect("FAILED to reset terminal state. Use `reset` to reinitialize terminal.");
        eprintln!("\nError: {e}")
    }
}

fn app(wordle: Wordle) -> Result<()> {
    
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {

            let mut guess = String::new();

            match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => break,
                (KeyCode::Char(c), _) => {
                    if guess.len() == 5 {
                        continue;
                    }

                    guess.push(c);
                    let character_hint = wordle.hint(&c);
                    io::stdout()
                        .execute(style::Print(c))?;
                },
                (KeyCode::Backspace, _) => {
                    return Err(anyhow!("Backspace unhandled"))
                },
                (KeyCode::Enter, _) => {
                    return Err(anyhow!("Enter unhandled"))
                }
                (_,_) => {},
            }
        }
    }

    io::stdout()
        .execute(cursor::MoveToPreviousLine(1))?
        .execute(terminal::Clear(ClearType::CurrentLine))?
        .execute(style::Print(format!("[done]")))?
        .execute(cursor::MoveToNextLine(1))?;

    disable_raw_mode()?;
    Ok(())
}