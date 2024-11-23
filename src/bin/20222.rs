use advent_of_code::load_data;
use std::{error::Error, fmt::Display, io::BufRead, str::FromStr, string::ParseError};

const ADVENT_NUM: &str = "20222";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut all_hands = Vec::new();
    for line in file.lines() {
        let hand: Hand = line?.parse()?;
        all_hands.push(hand);
        println!("{hand:?} => {}", hand.score());
    }
    let score = all_hands.iter().map(|h| h.score()).sum::<u32>();
    println!("Score => {score}");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Hand(RPS, RPS);

#[derive(Debug)]
struct HandParseError(String);

impl Display for HandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for HandParseError {}

impl FromStr for Hand {
    type Err = HandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((l, r)) = s.split_once(' ') {
            let left = match l {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => return Err(HandParseError("Unknown symbols".to_string())),
            };

            let right = match r {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                _ => return Err(HandParseError("Unknown symbols".to_string())),
            };

            Ok(Hand(left, right))
        } else {
            Err(HandParseError("Couldn't find space".to_string()))
        }
    }
}

impl Hand {
    fn score(&self) -> u32 {
        match (self.0, self.1) {
            (RPS::Rock, RPS::Rock) => 4,
            (RPS::Paper, RPS::Paper) => 5,
            (RPS::Scissors, RPS::Scissors) => 6,
            (RPS::Rock, RPS::Paper) => 8,
            (RPS::Paper, RPS::Scissors) => 9,
            (RPS::Scissors, RPS::Rock) => 7,
            (RPS::Rock, RPS::Scissors) => 3,
            (RPS::Paper, RPS::Rock) => 1,
            (RPS::Scissors, RPS::Paper) => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
