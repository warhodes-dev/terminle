use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub mod words;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Hint {
    Yellow,
    Green,
    NotIn,
}

#[derive(Default)]
pub struct Wordle {
    word: String,
    pub guesses: Vec<String>,
}

impl Wordle {
    pub fn new(challenge_number: usize) -> Self {
        let word = words::WORDS[challenge_number % words::WORDS_LEN].to_owned();
        Wordle { word, ..Default::default() }
    }

    /// Guess a string and add it to the list of guesses
    pub fn guess(&mut self, guess: &str) -> bool {
        assert_eq!(guess.len(), 5);
        if guess == &self.word {
            true // You win!
        } else {
            self.guesses.push(guess.to_owned());
            false
        }
    }

    /// Provide a hint for a single character against the solution
    pub fn hint(&self, char: char, pos: usize) -> Hint {
        if self.word.chars().nth(pos) == Some(char) {
            Hint::Green
        } else if self.word.contains(char) {
            Hint::Yellow
        } else {
            Hint::NotIn
        }
    }

    /// Provide a suggestion for a single character against all known hints
    pub fn suggest(&self, char: char, pos: usize) -> Option<Hint> {
        if self.is_known(char, pos) {
            Some(Hint::Green)
        } else if self.is_seen(char) && self.word.contains(char) {
            Some(Hint::Yellow)
        } else if self.is_seen(char) && !self.word.contains(char) {
            Some(Hint::NotIn)
        } else {
            None
        }
    }

    fn is_seen(&self, char: char) -> bool {
        for guess in self.guesses.iter() {
            if guess.contains(char) {
                return true;
            }
        }
        false
    }

    fn is_known(&self, char: char, pos: usize) -> bool {
        for guess in self.guesses.iter() {
            if guess.chars().nth(pos) == Some(char) 
            && self.word.chars().nth(pos) == Some(char) {
                return true;
            }
        }
        false
    }

}