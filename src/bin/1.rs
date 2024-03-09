use std::io::BufRead;

use advent_of_code::load_data;

const ADVENT_NUM: &str = "1";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let total: u32 = file
        .lines()
        .map(|s| find_nums(s.unwrap()))
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();
    println!("{total}");
}

fn find_nums(s: String) -> Vec<u32> {
    let tmp = s
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    println!("{tmp}");
    let intermediate: Vec<char> = tmp.chars().filter(|c| c.is_ascii_digit()).collect();
    intermediate
        .iter()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}
