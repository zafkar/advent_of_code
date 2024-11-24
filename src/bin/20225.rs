use advent_of_code::load_data;
use itertools::Itertools;
use regex::Regex;
use std::{
    error::Error,
    fmt::Display,
    io::{BufRead, Read},
    str::FromStr,
};

const ADVENT_NUM: &str = "20225";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let (stacks_text, moves_text) = match data.split_once("\r\n\r\n") {
        Some(a) => a,
        None => return Err(Box::new(GenericParseError("ALED".to_string()))),
    };

    let mut stacks = stacks_text.parse::<Stacks>()?;
    println!("{stacks}");

    let crane_moves = moves_text
        .lines()
        .map(|line| line.parse::<CraneMove>().unwrap())
        .collect_vec();

    for cranemove in crane_moves {
        println!("################");
        println!("{cranemove:?}");
        stacks.cranemove(&cranemove);
        println!("{stacks}");
    }

    println!("Top => {}", stacks.top());
    Ok(())
}

#[derive(Debug)]
struct CraneMove(usize, usize, usize);

impl FromStr for CraneMove {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = match Regex::new(r#"^move (\d+) from (\d+) to (\d+)$"#) {
            Ok(a) => a,
            Err(a) => return Err(GenericParseError(a.to_string())),
        };

        let caps = match re.captures(s) {
            Some(a) => a,
            None => return Err(GenericParseError("No matches".to_string())),
        };

        let num = match caps.get(1) {
            Some(a) => a.as_str().parse::<usize>().unwrap(),
            None => return Err(GenericParseError("No matches".to_string())),
        };

        let src = match caps.get(2) {
            Some(a) => a.as_str().parse::<usize>().unwrap(),
            None => return Err(GenericParseError("No matches".to_string())),
        };

        let dst = match caps.get(3) {
            Some(a) => a.as_str().parse::<usize>().unwrap(),
            None => return Err(GenericParseError("No matches".to_string())),
        };

        Ok(CraneMove(num, src, dst))
    }
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn highest(&self) -> Option<usize> {
        self.0.iter().map(|s| s.len()).max()
    }

    fn cranemove(&mut self, cranemove: &CraneMove) -> Result<(), ()> {
        for _ in 0..cranemove.0 {
            if let Some(c) = self.0[cranemove.1 - 1].pop() {
                self.0[cranemove.2 - 1].push(c);
            } else {
                return Err(());
            }
        }
        Ok(())
    }

    fn top(&self) -> String {
        self.0.iter().map(|s| s.last().unwrap_or(&'.')).collect()
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for current_height in (0..self.highest().unwrap_or(0)).rev() {
            for stack in self.0.iter() {
                if let Some(content) = stack.get(current_height) {
                    write!(f, "[{content}] ")?;
                } else {
                    write!(f, "    ")?;
                }
            }
            write!(f, "\n")?;
        }
        for i in 1..=self.0.len() {
            write!(f, " {i}  ",)?;
        }

        Ok(())
    }
}

impl FromStr for Stacks {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Stacks(Vec::new());
        let num_stacks = match s.lines().rev().next() {
            Some(last_line) => last_line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            None => return Err(GenericParseError("No last lines ?".to_string())),
        };
        for _ in 0..num_stacks {
            stacks.0.push(Vec::new());
        }

        for line in s.lines().rev().skip(1) {
            for index in 0..num_stacks {
                //println!("{}", &line[(index * 4) + 1..(index * 4 + 2)]);
                let content = &line[(index * 4) + 1..(index * 4 + 2)];
                if content == " " {
                    continue;
                }
                stacks.0[index].push(content.chars().next().unwrap());
            }
        }
        Ok(stacks)
    }
}

#[derive(Debug)]
struct GenericParseError(String);

impl Display for GenericParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GenericParseError {}
