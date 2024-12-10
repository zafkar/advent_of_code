use std::{error::Error, io::Read};

use advent_of_code::load_data;
use regex::Regex;
const ADVENT_NUM: &str = "2024/3";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mul_regex = Regex::new(r#"(?:(mul)\((\d{1,3}),(\d{1,3})\)|(do|don't)\(()()\))"#)?;
    let mut all_muls = vec![];
    let mut state = true;
    for caps in mul_regex.captures_iter(&text) {
        let (_, [op, left, right]) = caps.extract();
        match (op, left, right, state) {
            ("don't", _, _, _) => state = false,
            ("do", _, _, _) => state = true,
            ("mul", left, right, true) => {
                all_muls.push((left.parse::<i32>()?, right.parse::<i32>()?))
            }
            ("mul", _, _, false) => (),
            _ => panic!("ALED"),
        }
        println!("{},{},{}", op, left, right);
    }

    let result = all_muls.iter().fold(0, |acc, (l, r)| acc + l * r);
    println!("{result}");

    Ok(())
}
