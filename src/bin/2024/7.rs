use advent_of_code::load_data;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{error::Error, fmt::Display, io::BufRead, iter::zip, str::FromStr, time::Instant};

const ADVENT_NUM: &str = "2024/7";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let equations = file
        .lines()
        .map(|l| l.unwrap().parse::<Equation>().unwrap())
        .collect_vec();
    // for line in file.lines() {
    //     let row: Equation = line?.parse()?;
    //     println!("{row:?} => {:?}", row.can_be_valid());
    // }

    let part1_start = Instant::now();
    let result_part1: u64 = equations
        .par_iter()
        .filter(|e| e.can_be_valid())
        .map(|e| e.result)
        .sum();

    println!(
        "Result Part 1 => {} in {:?}",
        result_part1,
        part1_start.elapsed()
    );

    Ok(())
}

#[derive(Debug)]
struct Equation {
    result: u64,
    terms: Vec<u64>,
}

impl Equation {
    fn can_be_valid(&self) -> bool {
        for op_set in Operation::generate_all_combinations(self.terms.len() - 1) {
            if self.is_valid(op_set) {
                return true;
            }
        }
        false
    }

    fn is_valid(&self, operations: Vec<Operation>) -> bool {
        let mut acc = match self.terms.first() {
            Some(a) => *a,
            None => return false,
        };
        for (term, op) in zip(self.terms[1..].iter(), operations.iter()) {
            acc = match op {
                Operation::Add => acc + term,
                Operation::Mul => acc * term,
                Operation::Concat => concat(acc, term),
            };
            if acc > self.result {
                return false;
            }
        }
        acc == self.result
    }
}

fn concat(l: u64, r: &u64) -> u64 {
    (l.to_string() + &r.to_string()).parse().unwrap()
}

impl FromStr for Equation {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result_text, terms_text) = match s.split_once(':') {
            Some(a) => a,
            _ => return Err(GenericParseError::new("Couldn't split result from terms")),
        };

        let result = match result_text.parse() {
            Ok(a) => a,
            _ => return Err(GenericParseError::new("Couldn't parse result")),
        };

        let terms = terms_text
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .map(|t| t.parse().unwrap())
            .collect_vec();
        Ok(Equation { result, terms })
    }
}

#[derive(Debug)]
struct GenericParseError(String);

impl GenericParseError {
    fn new(msg: &str) -> GenericParseError {
        GenericParseError(msg.to_string())
    }
}

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn generate_all_combinations(
        n: usize,
    ) -> itertools::Combinations<
        std::iter::FlatMap<
            std::ops::Range<usize>,
            Vec<Operation>,
            impl FnMut(usize) -> Vec<Operation>,
        >,
    > {
        let single_set = Operation::one_off_all();
        (0..n)
            .flat_map(move |_| single_set.clone())
            .combinations(n)
    }

    fn one_off_all() -> Vec<Operation> {
        vec![Operation::Add, Operation::Mul, Operation::Concat]
    }
}
