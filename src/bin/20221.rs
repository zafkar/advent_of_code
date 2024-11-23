use advent_of_code::load_data;
use std::{error::Error, io::BufRead};

const ADVENT_NUM: &str = "20221";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut current_max = 0;
    let mut current_elf = 0;

    for line in file.lines() {
        let line = line?;

        if line.len() == 0 {
            current_max = current_max.max(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>()?;
        println!("Current line => {line}, current elf => {current_elf}");
    }
    println!("{}", current_max.max(current_elf));
    Ok(())
}
