use anyhow::{Result, anyhow};

pub mod words;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Hint {
    Gray,
    Yellow,
    Green,
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
    pub fn guess(&mut self, guess: String) -> Result<Guess> {
        if guess.len() != 5 {
            return Err(anyhow!("Guess must be exactly length 5"));
        }

        let solution = &self.word;
        let guess_result = guess.chars()
            .zip(solution.chars())
            .map(|(guess_char, solution_char)| {
                let hint = 
                    if guess_char == solution_char {
                        Hint::Green
                    } else if solution.contains(guess_char) {
                        Hint::Yellow
                    } else {
                        Hint::Gray
                    };
                (guess_char, hint)
            })
            .collect::<Guess>();

        self.guesses.push(guess_result.clone());
        Ok(guess_result)
    }

    /// Provide a hint for a single character
    pub fn hint(&self, char: &char) -> Option<&Hint> {
        self.guesses.iter()
            .flatten()
            .filter_map(|(c, hint)| (c == char).then_some(hint))
            .reduce(|dominant, hint| {
                std::cmp::max(dominant, hint)
            })
    }
}