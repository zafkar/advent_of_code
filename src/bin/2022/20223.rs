use advent_of_code::load_data;
use std::{error::Error, fmt::Display, io::BufRead, iter::zip, str::FromStr, time::Instant};

const ADVENT_NUM: &str = "2022/3";

fn char2priority(c: char) -> u32 {
    let integer: u32 = c.into();
    match integer {
        a if (97..=122).contains(&a) => a - 96,
        a if (65..=90).contains(&a) => a - 38,
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
    let rucksacks: Vec<Rucksack> = file
        .lines()
        .map(|line| line.unwrap().parse::<Rucksack>().unwrap())
        .collect();

    let result: u32 = zip(
        rucksacks.iter().step_by(3),
        zip(
            rucksacks.iter().skip(1).step_by(3),
            rucksacks.iter().skip(2).step_by(3),
        ),
    )
    .map(|(a, (b, c))| a.find_badge(b, c).unwrap())
    .map(char2priority)
    .sum();

    println!("{result}");
    println!("End => {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug)]
struct Rucksack(String);

impl Rucksack {
    fn find_badge(&self, other_a: &Rucksack, other_b: &Rucksack) -> Option<char> {
        for l in self.0.chars() {
            if !other_a.0.contains(l) {
                continue;
            }
            if other_b.0.contains(l) {
                return Some(l);
            }
        }
        None
    }
}

impl FromStr for Rucksack {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack(s.to_string()))
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
