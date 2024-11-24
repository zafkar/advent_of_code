use advent_of_code::load_data;
use std::{error::Error, fmt::Display, io::BufRead, str::FromStr};

const ADVENT_NUM: &str = "1";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    for line in file.lines() {
        let row: Row = line?.parse()?;
        println!("{row:?}");
    }
    Ok(())
}

#[derive(Debug)]
struct Row;

impl FromStr for Row {
    type Err = GenericParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Row)
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
