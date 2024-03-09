use std::io::BufRead;
use advent_of_code::load_data;

const ADVENT_NUM: &str = "1";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    for line in file.lines().map(|f| f.unwrap()) {
        println!("{line}");
    }
}
