use advent_of_code::{load_data, AdventError};
use std::{error::Error, io::BufRead, str::FromStr};

const ADVENT_NUM: &str = "2024/1";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "sample.txt")?;
    for line in file.lines() {
        let row: Row = line?.parse()?;
        println!("{row:?}");
    }
    Ok(())
}

#[derive(Debug)]
struct Row;

impl FromStr for Row {
    type Err = AdventError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Row)
    }
}
