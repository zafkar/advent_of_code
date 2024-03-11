use advent_of_code::load_data;
use regex::Regex;
use std::io::BufRead;

const ADVENT_NUM: &str = "3";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();

    let mut parsed_lines_nums = vec![];
    let mut parsed_lines_sym = vec![];
    for (index, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        println!("{line}");
        let (syms, ns) = parse_line(&line, index as i32);
        parsed_lines_nums.push(ns);
        parsed_lines_sym.push(syms);
    }

    for i in 0..parsed_lines_nums.len() {
        println!(
            "{:?} == {:?}",
            parsed_lines_nums.get(i),
            parsed_lines_sym.get(i)
        );
    }

    let mut sym_num_links: Vec<(Symbol, Vec<Number>)> = vec![];
    println!("Parsing done\n\n##############");
    for (index, sym_line) in parsed_lines_sym.iter().enumerate() {
        for sym in sym_line {
            let mut lowerbound = 0;
            if index > 1 {
                lowerbound = index - 1;
            }
            let mut higher_bound = parsed_lines_nums.len() - 1;
            if index < parsed_lines_nums.len() - 2 {
                higher_bound = index + 1;
            }
            let tied: Vec<Number> = parsed_lines_nums[lowerbound..=higher_bound]
                .iter()
                .flatten()
                .copied()
                .filter(|n| n.is_tied(*sym))
                .collect();

            println!("{sym:?} => {tied:?}");
            sym_num_links.push((*sym, tied));
        }
    }

    let total: i32 = sym_num_links.iter().fold(0i32, |acc, (_sym, arr)| {
        acc + arr.iter().map(|n| n.0).sum::<i32>()
    });

    println!("Total1 {total}");

    let total_gear: i32 = sym_num_links
        .iter()
        .filter(|(s, ns)| s.0 == '*' && ns.len() == 2)
        .fold(0i32, |acc, (_s, ns)| acc + ns[0].0 * ns[1].0);

    println!("Total_Gear => {total_gear}");
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Symbol(char, Pos);

#[derive(Debug, Clone, Copy)]
struct Number(i32, Pos, Pos);

impl Number {
    fn is_tied(&self, sym: Symbol) -> bool {
        (self.1.y <= sym.1.y + 1)
            && (self.1.y >= sym.1.y - 1)
            && (self.2.x >= sym.1.x - 1)
            && (self.1.x <= sym.1.x + 1)
    }
}

const NUM_REGEX: &str = r"[0-9]+";
const SYM_REGEX: &str = r"[^0-9.]";

fn parse_line(s: &str, current_line_number: i32) -> (Vec<Symbol>, Vec<Number>) {
    let mut result_nums = vec![];
    let mut result_sym = vec![];

    let num_regex = Regex::new(NUM_REGEX).unwrap();
    for m in num_regex.find_iter(s) {
        result_nums.push(Number(
            m.as_str().parse().unwrap(),
            Pos {
                y: current_line_number,
                x: m.start() as i32,
            },
            Pos {
                y: current_line_number,
                x: m.end() as i32 - 1,
            },
        ))
    }

    let sym_regex = Regex::new(SYM_REGEX).unwrap();
    for m in sym_regex.find_iter(s) {
        result_sym.push(Symbol(
            m.as_str().chars().next().unwrap(),
            Pos {
                y: current_line_number,
                x: m.start() as i32,
            },
        ))
    }

    (result_sym, result_nums)
}
