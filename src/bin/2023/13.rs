use advent_of_code::load_data;
use std::{error::Error, fmt::Display, io::BufRead};

const ADVENT_NUM: &str = "2023/13";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    let mut current_pat = vec![];
    let mut patterns = vec![];
    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            patterns.push(Pattern(current_pat));
            current_pat = vec![];
            continue;
        }
        current_pat.push(line.clone());
    }
    if !current_pat.is_empty() {
        patterns.push(Pattern(current_pat));
    }

    let mut acc = 0;
    for p in patterns.iter() {
        println!("{p}");
        println!("Vertical ?");
        for va in p.ver_sym() {
            println!("Vertical axis {}", va);
            acc += va;
        }
        println!("Horizontal ?");
        for ha in p.hor_sym() {
            println!("Horizontal axis {}", ha);
            acc += 100 * ha;
        }
        //stdin().read_line(&mut String::new())?;
    }
    println!("Total {acc}");
    Ok(())
}

#[derive(Debug)]
struct Pattern(Vec<String>);

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

fn sym(row: &String, candidates: Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    'candidate_loop: for candidate_index in candidates {
        let (left, right) = row.split_at(candidate_index);
        'sym_loop: for (i, c) in right.char_indices() {
            if left.len() > i {
                match left.chars().nth(left.len() - 1 - i) {
                    Some(b) if b != c => continue 'candidate_loop,
                    _ => (),
                }
            } else {
                break 'sym_loop;
            }
        }
        //println!("{left}|{right}");
        result.push(candidate_index);
    }
    result
}

fn full_sym(rows: &Vec<String>) -> Vec<usize> {
    let mut current_candidates = vec![];
    if let Some(row) = rows.first() {
        current_candidates = sym(row, (1..row.len()).collect());
    }
    for line in rows.iter() {
        current_candidates = sym(line, current_candidates);
        if current_candidates.is_empty() {
            return vec![];
        }
    }
    current_candidates
}

impl Pattern {
    fn ver_sym(&self) -> Vec<usize> {
        full_sym(&self.0)
    }

    fn width(&self) -> usize {
        if let Some(first) = self.0.first() {
            first.len()
        } else {
            0
        }
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn transpose(&self) -> Pattern {
        let mut full = vec![String::new(); self.width()];
        for column in 0..self.height() {
            for (index, line) in full.iter_mut().enumerate() {
                if let Some(src) = self.0.get(column) {
                    if let Some(ch) = src.chars().nth(index) {
                        line.push(ch);
                    }
                }
            }
        }
        Pattern(full)
    }

    fn hor_sym(&self) -> Vec<usize> {
        self.transpose().ver_sym()
    }
}
