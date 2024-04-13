use advent_of_code::load_data;
use std::{io::BufRead, str::FromStr};

const ADVENT_NUM: &str = "12";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let mut acc = 0;
    for line in file.lines().map(|f| f.unwrap()) {
        let n: Nonogram = line.parse().unwrap();
        let ps = n.generate_all_possibilities();
        acc += ps.len();
        println!("{n:?} => {}", ps.len());
        println!("{ps:?}");
        println!();
    }
    println!("Total {acc}");
}

#[derive(Debug)]
struct Nonogram {
    line: String,
    hint: Vec<u32>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Nonogram {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, hint_txt) = match s.split_once(' ') {
            Some((a, b)) => (a.to_string(), b),
            None => return Err(ParseError),
        };

        let hint = match hint_txt.split(',').map(|h| h.parse()).collect() {
            Ok(a) => a,
            Err(_) => return Err(ParseError),
        };

        Ok(Nonogram { line, hint })
    }
}

fn add_to_all(mut list: Vec<String>, ch: char) -> Vec<String> {
    for i in list.iter_mut() {
        i.push(ch);
    }
    list
}

impl Nonogram {
    fn is_valid(&self) -> bool {
        let rebuilt_hint = self.rebuild_hint();
        if self.hint.len() != rebuilt_hint.len() {
            return false;
        }
        for (i, h) in self.hint.iter().enumerate() {
            if let Some(true_hint) = self.rebuild_hint().get(i) {
                if true_hint != h {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.line.is_empty()
    }

    fn generate_all_possibilities(&self) -> Vec<Nonogram> {
        let mut all_lines: Vec<String> = vec!["".to_string()];
        for c in self.line.chars() {
            match c {
                '#' => all_lines = add_to_all(all_lines, '#'),
                '.' => all_lines = add_to_all(all_lines, '.'),
                '?' => {
                    let dup = add_to_all(all_lines.clone(), '#');
                    all_lines = add_to_all(all_lines, '.');
                    all_lines.extend(dup);
                }
                _ => (),
            }
        }

        all_lines
            .iter()
            .map(|p| Nonogram {
                hint: self.hint.clone(),
                line: p.to_string(),
            })
            .filter(|n| n.is_valid())
            .collect()
    }

    fn rebuild_hint(&self) -> Vec<u32> {
        let mut hint = vec![];
        let mut current_chain = 0;
        for c in self.line.chars() {
            match c {
                '#' => current_chain += 1,
                '.' => {
                    if current_chain > 0 {
                        hint.push(current_chain);
                        current_chain = 0;
                    }
                }
                _ => unimplemented!("Unrecognized char"),
            };
        }
        if current_chain > 0 {
            hint.push(current_chain);
        }
        hint
    }
}
