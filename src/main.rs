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

#[derive(Clone, Copy)]
enum Status {
    InvalidWord,
    InvalidLength,
    AlreadyGuessed,
}

impl Status {
    fn msg(&self) -> &str {
        match self {
            Status::InvalidWord => "✗ Invalid word",
            Status::InvalidLength => "✗ Must be 5 characters long",
            Status::AlreadyGuessed => "✗ Already guessed",
        }
    }
}

fn app(mut wordle: Wordle) -> Result<()> {
    
    terminal::enable_raw_mode()?;

    let mut guess = String::new();
    let mut status: Option<Status> = None;
    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {

            if status.is_some() {
                clear_status()?;
            }

            match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => break,
                (KeyCode::Char(char), _) => handle_character(&wordle, &mut guess, char)?,
                (KeyCode::Backspace, _) => handle_backspace(&mut guess)?,
                (KeyCode::Enter, _) => { status = handle_enter(&mut wordle, &mut guess)? },
                (_,_) => {},
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn handle_character(wordle: &Wordle, guess: &mut String, char: char) -> Result<()> {
    if guess.len() == 5 {
        return Ok(());
    }

    guess.push(char);
    let suggest = wordle.suggest(char, guess.len() - 1);
    let color = match suggest {
        Some(Hint::Green) => style::Color::DarkGreen,
        Some(Hint::Yellow) => style::Color::Yellow,
        Some(Hint::NotIn) => style::Color::DarkRed,
        None => style::Color::Reset,
    };

    io::stdout()
        .execute(style::SetForegroundColor(color))?
        .execute(style::Print(char))?;
    Ok(())
}

fn handle_backspace(guess: &mut String) -> Result<()> {
    if guess.len() == 0 {
        return Ok(());
    }

    guess.pop();
    io::stdout()
        .execute(cursor::MoveLeft(1))?
        .execute(style::Print(' '))?
        .execute(cursor::MoveLeft(1))?;
    Ok(())
}

fn handle_enter(wordle: &mut Wordle, guess: &mut String) -> Result<Option<Status>> {

    //TODO: Move this logic to wordle.rs
    let status = 
        if guess.len() != 5 {
            Some(Status::InvalidLength)
        } else if !wordle::words::VALID.contains(&guess) {
            Some(Status::InvalidWord)
        } else if wordle.guesses.contains(&guess) {
            Some(Status::AlreadyGuessed)
        } else {
            None
        };

    if status.is_some() {
        print_status(status.unwrap())?;
        return Ok(status)
    }

    let result = wordle.guess(&guess);

    io::stdout()
        .execute(cursor::MoveToColumn(0))?;

    for (pos, char) in guess.chars().enumerate() {
        let hint = wordle.hint(char, pos);
        let (fg, bg) = match hint {
            Hint::Green => (style::Color::Black, style::Color::DarkGreen),
            Hint::Yellow => (style::Color::Black, style::Color::DarkYellow),
            Hint::NotIn => (style::Color::Reset, style::Color::Reset),
        };
        io::stdout()
            .execute(style::SetForegroundColor(fg))?
            .execute(style::SetBackgroundColor(bg))?
            .execute(style::Print(char))?;
    }

    io::stdout()
        .execute(cursor::MoveToColumn(0))?
        .execute(style::SetForegroundColor(style::Color::Reset))?
        .execute(style::SetBackgroundColor(style::Color::Reset))?
        .execute(style::Print('\n'))?
        .execute(cursor::MoveToColumn(0))?;

    guess.clear();
    Ok(None)
}

fn print_status(status: Status) -> Result<()> {
    io::stdout()
        .execute(cursor::SavePosition)?
        .execute(cursor::MoveToColumn(8))?
        .execute(style::SetForegroundColor(style::Color::Red))?
        .execute(style::Print(status.msg()))?
        .execute(cursor::RestorePosition)?
        ;
    Ok(())
}

fn clear_status() -> Result<()> {
    io::stdout()
        .execute(cursor::SavePosition)?
        .execute(cursor::MoveToColumn(8))?
        .execute(terminal::Clear(terminal::ClearType::UntilNewLine))?
        .execute(cursor::RestorePosition)?
        ;
    Ok(())
}