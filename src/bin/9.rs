use advent_of_code::load_data;
use std::io::BufRead;
use std::iter::zip;

const ADVENT_NUM: &str = "9";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let mut results = vec![];
    for line in file.lines().map(|l| l.unwrap()) {
        let readings: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        println!("=> {:?}", readings);
        let result = descent(&readings);
        results.push(readings.first().unwrap() - result);
        println!("Result => {}", results.first().unwrap());
    }

    let total: i32 = results.iter().sum();
    println!("Final => {total}");
}

fn descent(nums: &Vec<i32>) -> i32 {
    let next_step: Vec<i32> = zip(nums[1..].iter(), nums[..nums.len() - 1].iter())
        .map(|(a, b)| a - b)
        .collect();
    println!("{:?}", next_step);
    if next_step.iter().fold(0, |a, e| a + e) != 0i32 {
        let d_result = descent(&next_step);
        let new_num = match next_step.first() {
            Some(a) => a.clone(),
            _ => 0,
        };
        println!("step => {new_num}, {d_result},",);
        return new_num - d_result;
    }
    println!("as deep as ^pssoible");
    0
}
