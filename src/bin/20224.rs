use advent_of_code::load_data;
use itertools::Itertools;
use std::{error::Error, fmt::Display, io::BufRead, num::ParseIntError, str::FromStr};

const ADVENT_NUM: &str = "20224";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut count: u32 = 0;
    for line in file.lines() {
        let row: Row = line?.parse()?;
        println!("{row:?}, Overlapped? {:?}", row.is_overlapped());
        if row.is_overlapped() {
            count += 1;
        }
    }
    // let result: usize = file
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .map(|line| line.parse::<Row>().unwrap())
    //     .map(|r| r.is_fully_overlapped())
    //     .filter(|r| *r)
    //     .count();
    println!("Result => {count}");
    Ok(())
}

#[derive(Debug)]
struct Pair(u32, u32);

#[derive(Debug)]
struct Row(Pair, Pair);

impl FromStr for Row {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let integer_text = s.split(&[',', '-']).collect_vec();
        Ok(Row(
            Pair(integer_text[0].parse()?, integer_text[1].parse()?),
            Pair(integer_text[2].parse()?, integer_text[3].parse()?),
        ))
    }
}

impl Row {
    #[allow(dead_code)]
    fn is_fully_overlapped(&self) -> bool {
        self.0.contain(&self.1) || self.1.contain(&self.0)
    }

    fn is_overlapped(&self) -> bool {
        self.0.overlap(&self.1)
    }
}

impl Pair {
    fn contain(&self, other: &Pair) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlap(&self, other: &Pair) -> bool {
        !(self.0 > other.1 || self.1 < other.0)
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
