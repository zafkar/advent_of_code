use advent_of_code::load_data;
use itertools::{Combinations, Itertools};
use std::{io::BufRead, iter::zip, ops::Range, str::FromStr, sync::mpsc::channel};

const ADVENT_NUM: &str = "12";

fn main() {
    let file = load_data(ADVENT_NUM, "sample.txt").unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let (tx, rx) = channel();
    for line in file.lines().map(|f| f.unwrap()) {
        let unfolded_nonogram: Nonogram = line.parse::<Nonogram>().unwrap().unfold();
        let new_tx = tx.clone();
        pool.spawn(move || {
            let mut total = 0;
            for n in unfolded_nonogram.possibilities_iter().unwrap() {
                new_tx.send(n).unwrap();
                total += 1;
            }
            println!("{unfolded_nonogram:?} => {total}");
        });
    }
    drop(tx);

    let count = rx.iter().count();
    println!("Total {count}");
}

#[derive(Debug, Clone)]
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

impl Nonogram {
    fn unfold(&self) -> Nonogram {
        let mut new_line = String::new();
        let mut new_hints = vec![];

        for _ in 0..5 {
            new_line.push_str(&self.line);
            new_line.push('?');
            new_hints.append(&mut self.hint.clone());
        }
        new_line = new_line[..new_line.len() - 1].to_string();

        Nonogram {
            line: new_line,
            hint: new_hints,
        }
    }

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

    fn equivalent(&self, other: &Nonogram) -> bool {
        for (a, b) in zip(self.line.chars(), other.line.chars()) {
            match (a, b) {
                ('.', '#') | ('#', '.') => return false,
                _ => (),
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.line.is_empty()
    }

    fn hint_sum(&self) -> u32 {
        self.hint.iter().sum()
    }

    fn possibilities_iter(&self) -> Option<NonogramPossibilitiesIterator> {
        let gaps =
            match self.line.len() as i32 - self.hint_sum() as i32 - self.hint.len() as i32 + 1 {
                a if a < 0 => return None,
                a => a as u32,
            };
        let positions_possibilities = gaps + self.hint.len() as u32;
        Some(NonogramPossibilitiesIterator(
            self.clone(),
            (0..positions_possibilities).combinations(self.hint.len()),
        ))
    }

    fn generate_all_possibilities(&self) -> Vec<Nonogram> {
        let mut all_lines: Vec<String> = vec![];
        let gaps =
            match self.line.len() as i32 - self.hint_sum() as i32 - self.hint.len() as i32 + 1 {
                a if a < 0 => return vec![],
                a => a as u32,
            };

        let positions_possibilities = gaps + self.hint.len() as u32;
        for comb in (0..positions_possibilities).combinations(self.hint.len()) {
            let mut current_offset = 0;
            let mut current_line = String::new();
            for (pos, hint) in zip(comb, self.hint.clone()) {
                let num_of_gap_to_add = pos - current_offset;
                for _ in 0..num_of_gap_to_add {
                    current_line.push('.')
                }
                current_offset += num_of_gap_to_add + 1;

                for _ in 0..hint {
                    current_line.push('#');
                }
                current_line.push('.');
            }
            current_line = current_line[..current_line.len() - 1].to_string();
            let padding = match self.line.len() as i32 - current_line.len() as i32 {
                a if a < 0 => {
                    println!("{} => {}", self.line, current_line);
                    unimplemented!("KO");
                }
                a => a as u32,
            };
            for _ in 0..padding {
                current_line.push('.');
            }
            all_lines.push(current_line);
        }

        all_lines
            .iter()
            .map(|p| Nonogram {
                hint: self.hint.clone(),
                line: p.to_string(),
            })
            .filter(|n| n.equivalent(self))
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

struct NonogramPossibilitiesIterator(Nonogram, Combinations<Range<u32>>);

impl Iterator for NonogramPossibilitiesIterator {
    type Item = Nonogram;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let comb = match self.1.next() {
                Some(a) => a,
                None => return None,
            };
            let mut current_offset = 0;
            let mut current_line = String::new();
            for (pos, hint) in zip(comb, self.0.hint.clone()) {
                let num_of_gap_to_add = pos - current_offset;
                for _ in 0..num_of_gap_to_add {
                    current_line.push('.')
                }
                current_offset += num_of_gap_to_add + 1;

                for _ in 0..hint {
                    current_line.push('#');
                }
                current_line.push('.');
            }
            current_line = current_line[..current_line.len() - 1].to_string();
            let padding = match self.0.line.len() as i32 - current_line.len() as i32 {
                a if a < 0 => {
                    println!("{} => {}", self.0.line, current_line);
                    unimplemented!("KO");
                }
                a => a as u32,
            };
            for _ in 0..padding {
                current_line.push('.');
            }
            let candidate = Nonogram {
                hint: self.0.hint.clone(),
                line: current_line,
            };

            match self.0.equivalent(&candidate) {
                true => return Some(candidate),
                false => continue,
            }
        }
    }
}
