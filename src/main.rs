use advent_of_code::load_data;
use std::{error::Error, io::BufRead};

const ADVENT_NUM: &str = "1";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    for line in file.lines() {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}
