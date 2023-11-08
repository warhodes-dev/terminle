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

fn app(mut wordle: Wordle) -> Result<()> {
    
    terminal::enable_raw_mode()?;

    let mut guess = String::new();
    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {

            match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => break,
                (KeyCode::Char(c), _) => {
                    if guess.len() == 5 {
                        continue;
                    }

                    guess.push(c);
                    let hint = wordle.hint(&c, guess.len() - 1);
                    let color = match hint {
                        None => style::Color::Reset,
                        Some(Hint::Green(_)) => style::Color::Green,
                        Some(Hint::Yellow) => style::Color::Yellow,
                        Some(Hint::Gray) => style::Color::Grey,
                    };
                    io::stdout()
                        .execute(style::SetForegroundColor(color))?
                        .execute(style::Print(c))?;
                },
                (KeyCode::Backspace, _) => {
                    if guess.len() == 0 {
                        continue;
                    }

                    guess.pop();
                    io::stdout()
                        .execute(cursor::MoveLeft(1))?
                        .execute(style::Print(' '))?
                        .execute(cursor::MoveLeft(1))?;
                },
                (KeyCode::Enter, _) => {
                    if guess.len() != 5 {
                        continue;
                    }

                    let result = wordle.guess(&guess);

                    io::stdout()
                        .execute(cursor::MoveToColumn(0))?;

                    for (c, hint) in result {
                        let (fg, bg) = match hint {
                            Hint::Green(_) => (style::Color::Black, style::Color::DarkGreen),
                            Hint::Yellow => (style::Color::Black, style::Color::DarkYellow),
                            Hint::Gray => (style::Color::White, style::Color::DarkGrey),
                        };
                        io::stdout()
                            .execute(style::SetForegroundColor(fg))?
                            .execute(style::SetBackgroundColor(bg))?
                            .execute(style::Print(c))?;
                    }

                    io::stdout()
                        .execute(style::SetForegroundColor(style::Color::Reset))?
                        .execute(style::SetBackgroundColor(style::Color::Reset))?
                        .execute(style::Print('\n'))?
                        .execute(cursor::MoveToColumn(0))?;

                    guess.clear();
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