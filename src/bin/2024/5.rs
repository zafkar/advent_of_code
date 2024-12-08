use advent_of_code::load_data;
use itertools::Itertools;
use std::{collections::HashMap, error::Error, fmt::Display, io::Read};

const ADVENT_NUM: &str = "2024/5";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let (rules_text, update_text) = match full_text.split_once("\r\n\r\n") {
        Some(a) => a,
        None => {
            return Err(Box::new(GenericParseError(
                "can't find middle of file".to_string(),
            )))
        }
    };

    let mut rules = HashMap::new();
    for rule_line in rules_text.lines() {
        let (k, v) = match rule_line.split_once('|') {
            Some(a) => a,
            None => return Err(Box::new(GenericParseError("can't parse rule".to_string()))),
        };
        rules
            .entry(k.parse::<u32>()?)
            .or_insert(vec![])
            .push(v.parse::<u32>()?);
    }

    let updates = update_text
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    println!("Rules => {:?}", rules);
    println!("updates => {:?}", updates);

    for update in updates.iter() {
        println!("{update:?} => {}", check_order(update, &rules));
    }

    let part1_result = updates
        .iter()
        .filter(|update| check_order(update, &rules))
        .map(|update| update[(update.len() - 1) / 2])
        .sum::<u32>();

    println!("Part1 => {part1_result}");

    let part2_result = updates
        .iter()
        .filter(|update| !check_order(update, &rules))
        .map(|update| reorder(update, &rules)[(update.len() - 1) / 2])
        .sum::<u32>();

    println!("Part2 => {part2_result}");

    Ok(())
}

fn check_order(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (index, item) in update.iter().enumerate() {
        let empty_vec = vec![];
        let must_be_before = match rules.get(item) {
            Some(a) => a,
            None => &empty_vec,
        };

        for item_after in update[..index].iter() {
            if must_be_before.contains(item_after) {
                // println!(
                //     "Update => {update:?}, item => {item}, item_after => {item_after}, slice => {:?}, must_be_before => {must_be_before:?}",
                //     update[..index].iter().collect_vec()
                // );
                return false;
            }
        }
    }
    true
}

fn reorder(update_src: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut update = update_src.clone();
    'outer: while !check_order(&update, rules) {
        for (index, item) in update.iter().enumerate() {
            let empty_vec = vec![];
            let must_be_before = match rules.get(item) {
                Some(a) => a,
                None => &empty_vec,
            };

            for (indexafter, item_after) in update[..index].iter().enumerate() {
                if must_be_before.contains(item_after) {
                    update.swap(index, indexafter);
                    continue 'outer;
                }
            }
        }
    }
    update
}

#[derive(Debug)]
struct GenericParseError(String);

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}
