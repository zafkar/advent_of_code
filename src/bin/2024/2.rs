use std::{error::Error, io::BufRead, iter::zip};

use advent_of_code::load_data;

const ADVENT_NUM: &str = "2024/2";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;

    let reports = file
        .lines()
        .map(|l| {
            l.unwrap()
                .split_ascii_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    for r in reports.iter() {
        if !problem_dampener(&r) {
            //println!("{r:?} => {}", problem_dampener(&r))
        }
    }

    let result = reports.iter().filter(|r| problem_dampener(r)).count();
    println!("result => {result}");

    Ok(())
}

fn problem_dampener(r: &Vec<i64>) -> bool {
    if is_report_safe(r) {
        return true;
    }

    for (index, _) in r.iter().enumerate() {
        let mut test_vec = r.clone();
        test_vec.remove(index);
        if is_report_safe(&test_vec) {
            return true;
        }
    }

    false
}

fn is_report_safe(r: &Vec<i64>) -> bool {
    let mut is_asc = true;
    let mut is_desc = true;
    for (item, next_item) in zip(r[0..r.len() - 1].iter(), r[1..].iter()) {
        if item < next_item {
            is_desc = false;
        }

        if item > next_item {
            is_asc = false;
        }

        let item_diff = item.abs_diff(*next_item);
        if item_diff > 3 || item_diff < 1 {
            return false;
        }

        if !(is_asc || is_desc) {
            return false;
        }
    }
    true
}
