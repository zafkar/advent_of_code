use advent_of_code::load_data;
use itertools::Itertools;
use std::{error::Error, fmt::Display, io::BufRead, iter::zip, str::FromStr};

const ADVENT_NUM: &str = "2023/6";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let data = file
        .lines()
        .map(|s| {
            s.unwrap()
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .skip(1)
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let races = zip(data[0].clone(), data[1].clone())
        .map(|(time, distance)| {
            Race::new(time)
                .all_possibilities()
                .iter()
                .filter(|d| distance < **d)
                .count()
        })
        .fold(1, |acc, possibilites| acc * possibilites);

    println!("{races:?}");
    Ok(())
}

#[derive(Debug)]
struct Race(u32);

impl Race {
    fn new(time: u32) -> Race {
        Race(time)
    }

    fn distance(&self, pressed: u32) -> u32 {
        (self.0 - pressed) * pressed
    }

    fn all_possibilities(&self) -> Vec<u32> {
        (0..self.0).map(|t| self.distance(t)).collect_vec()
    }
}

#[derive(Debug)]
struct GenericParseError(String);

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}
