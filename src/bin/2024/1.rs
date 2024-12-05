use advent_of_code::load_data;
use itertools::Itertools;
use std::{collections::HashMap, error::Error, fmt::Display, io::BufRead, iter::zip};

const ADVENT_NUM: &str = "2024/1";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut left = vec![];
    let mut right = vec![];
    for line in file.lines() {
        let row = line?;
        println!("row => {row}");
        let (left_field, right_field) = match row
            .split_ascii_whitespace()
            .filter(|f| !f.is_empty())
            .map(|f| f.parse::<u32>().unwrap())
            .collect_tuple()
        {
            Some(a) => a,
            None => return Err(Box::new(GenericParseError("No collect".to_string()))),
        };

        left.push(left_field);
        right.push(right_field);
    }

    println!("left => {left:?}");
    println!("right => {right:?}");
    left.sort();
    right.sort();
    let result: u32 = zip(&left, &right).map(|(a, b)| a.abs_diff(*b)).sum();
    println!("{result}");

    println!("Part 2");
    let right_count = count_element(&right);
    let part2_result = left
        .clone()
        .iter()
        .map(|a| a * right_count.get(a).unwrap_or(&0))
        .sum::<u32>();
    println!("{part2_result:?}");
    Ok(())
}

fn count_element(v: &Vec<u32>) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    for item in v {
        *result.entry(*item).or_insert(0) += 1;
    }
    result
}

#[derive(Debug)]
struct GenericParseError(String);

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}
