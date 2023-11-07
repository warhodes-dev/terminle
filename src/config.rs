use cli::{Cli, ChallengeArg, NthArgs};
use clap::Parser;
use rand::Rng;

pub enum Challenge {
    Daily(usize),
    Random(usize),
    Nth(usize),
}

/// Public facing config
//  Overengineering is my passion
pub struct Config {
    pub challenge_number: usize,
}

impl Config {
    pub fn parse() -> Self {
        let cli = Cli::parse();
        let challenge_number = match cli.challenge {
            Some(ChallengeArg::Daily) | None => get_n_from_date(),
            Some(ChallengeArg::Random) => rand::thread_rng().gen_range(0..crate::wordle::words::WORDS_LEN),
            Some(ChallengeArg::Nth(NthArgs{ n })) => n as usize,
        };

        Config {
            challenge_number
        }
    }
}

mod cli {
    use clap::{Args, Parser, Subcommand};
    use crate::wordle::words::WORDS_LEN;

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub challenge: Option<ChallengeArg>,
    }

    #[derive(Copy, Clone, Subcommand)]
    pub enum ChallengeArg {
        /// [default] Get today's challenge
        Daily,
        /// Pick a random challenge
        Random,
        /// Specify a challenge (0 - 2308)
        Nth(NthArgs),
    }

    #[derive(Copy, Clone, Args)]
    pub struct NthArgs {
        #[arg(value_parser = clap::value_parser!(u32).range(0..(WORDS_LEN as i64)))]
        pub n: u32
    }
}

fn get_n_from_date() -> usize {
    let first_day = chrono::NaiveDate::from_ymd_opt(2023, 10, 22).unwrap();
    let current_day = chrono::Local::now().date_naive();
    (current_day - first_day).num_days() as usize
}