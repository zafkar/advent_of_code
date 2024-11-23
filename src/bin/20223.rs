use advent_of_code::load_data;
use itertools::Itertools;
use std::{error::Error, fmt::Display, io::BufRead, str::FromStr, time::Instant};

const ADVENT_NUM: &str = "20223";

fn char2priority(c: char) -> u32 {
    let integer: u32 = c.into();
    match integer {
        a if a <= 122 && a >= 97 => a - 96,
        a if a <= 90 && a >= 65 => a - 38,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = load_data(ADVENT_NUM, "input.txt")?;
    // for line in file.lines() {
    //     let rucksack: Rucksack = line?.parse()?;
    //     println!(
    //         "{rucksack:?} => {:?} => {}",
    //         rucksack.find_pair(),
    //         char2priority(rucksack.find_pair().unwrap())
    //     );
    // }
    let result: u32 = file
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<Rucksack>()
                .unwrap()
                .find_pair()
                .unwrap()
        })
        .map(|c| char2priority(c))
        .sum();
    println!("{result}");
    println!("End => {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug)]
struct Rucksack(String, String);

impl Rucksack {
    fn find_pair(&self) -> Option<char> {
        for l in self.0.chars().unique() {
            if self.1.contains(l) {
                return Some(l);
            }
        }
        None
    }
}

impl FromStr for Rucksack {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(s.len() / 2);
        Ok(Rucksack(left.to_string(), right.to_string()))
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
