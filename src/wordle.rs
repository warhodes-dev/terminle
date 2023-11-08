use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub mod words;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Hint {
    Gray,
    Yellow,
    Green(usize),
}

type Guess = Vec<(char, Hint)>;

#[derive(Default)]
pub struct Wordle {
    word: String,
    guesses: Vec<Guess>,
}

impl Wordle {
    pub fn new(challenge_number: usize) -> Self {
        let word = words::WORDS[challenge_number].to_owned();
        Wordle { word, ..Default::default() }
    }

    /// Guess a string and add it to the list of guesses
    pub fn guess(&mut self, guess: &str) -> Guess {
        assert_eq!(guess.len(), 5);

        let solution = &self.word;
        let guess_result = guess.chars()
            .zip(solution.chars())
            .enumerate()
            .map(|(idx, (guess_char, solution_char))| {
                let hint = 
                    if guess_char == solution_char {
                        Hint::Green(idx)
                    } else if solution.contains(guess_char) {
                        Hint::Yellow
                    } else {
                        Hint::Gray
                    };
                (guess_char, hint)
            })
            .collect::<Guess>();

        self.guesses.push(guess_result.clone());
        guess_result
    }

    /// Provide a hint for a single character
    pub fn hint(&self, char: &char, pos: usize) -> Option<Hint> {
        let mut overall_hint = None;

        let known_hints = self.guesses.iter()
            .flatten()
            .filter_map(|(c, hint)| (c == char).then_some(*hint));

        for hint in known_hints.rev() {
            if let Hint::Green(idx) = hint {
                if idx == pos {
                    overall_hint = Some(hint);
                    break;
                } else {
                    overall_hint = Some(Hint::Yellow)
                }
            } else if hint == Hint::Yellow {
                overall_hint = Some(hint);
                break;
            } else if hint == Hint::Gray {
                overall_hint = Some(hint);
            }
        }

        overall_hint
    }
}