use advent_of_code::load_data;
use std::{error::Error, io::BufRead};

const ADVENT_NUM: &str = "2022/1";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut current_elf = 0;
    let mut elves = Vec::new();

    for line in file.lines() {
        let line = line?;

        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<i32>()?;
        println!("Current line => {line}, current elf => {current_elf}");
    }
    println!("{}", elves.iter().max().unwrap());

    elves.sort();
    println!("{}", elves.iter().rev().take(3).sum::<i32>());
    Ok(())
}
