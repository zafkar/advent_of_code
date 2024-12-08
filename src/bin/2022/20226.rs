use advent_of_code::load_data;
use itertools::Itertools;
use std::{error::Error, io::Read};

const ADVENT_NUM: &str = "2022/6";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "sample.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    for i in 0..data.len() - 14 {
        if data[i..i + 14].chars().all_unique() {
            println!("{}", i + 14);
            break;
        }
    }
    Ok(())
}
