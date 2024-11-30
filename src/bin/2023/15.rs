use advent_of_code::load_data;
use regex::Regex;
use std::{error::Error, fmt::Display, io::BufRead, str::FromStr};

const ADVENT_NUM: &str = "2023/15";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "sample.txt")?;

    let mut hashmap = HashMap::new();
    for line in file.split(b',') {
        let line = String::from_utf8(line?)?;
        let ins: Instruction = line.parse()?;
        let mut hash = AdventHash::new();
        hash.push_str(&ins.label);
        match ins.action {
            InstructionAction::Append => {
                hashmap.insert(hash.0, ins.focal);
            }
            InstructionAction::Subtract => {}
        }
        println!("{} => {}", line, hash.0);
    }
    Ok(())
}

struct HashMap<T>(Vec<(u8, T)>);

impl HashMap<Lens> {
    fn new() -> HashMap<Lens> {
        HashMap(vec![])
    }
}
//Voir un peu l'organisation de donné dans hashmap classique

#[derive(Debug, Clone, Copy)]
struct AdventHash(u8);

impl AdventHash {
    fn new() -> AdventHash {
        AdventHash(0)
    }

    fn push(&mut self, c: char) -> () {
        self.0 = ((self.0 as u16 + (c as u16)) * 17 % 256) as u8;
    }

    fn push_vec(&mut self, bytes: &[u8]) -> () {
        for b in bytes {
            if *b == b'\n' {
                continue;
            }
            self.push(*b as char);
        }
    }

    fn push_str(&mut self, s: &str) -> () {
        self.push_vec(s.as_bytes())
    }
}

enum InstructionAction {
    Append,
    Subtract,
}

struct Instruction {
    label: String,
    action: InstructionAction,
    focal: Option<u8>,
}

#[derive(Debug)]
struct InstructionParseError;

impl Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing Instruction")
    }
}

impl Error for InstructionParseError {}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"(?P<label>[^=,-]+)(?P<action>=|-)(?P<focal>\d+)?").unwrap();

        let caps = match reg.captures(s) {
            Some(a) => a,
            None => return Err(InstructionParseError),
        };

        let label = match caps.name("label") {
            Some(a) => a.as_str().to_string(),
            None => return Err(InstructionParseError),
        };

        let action = match caps.name("action") {
            Some(a) => match a.as_str() {
                "=" => InstructionAction::Append,
                "-" => InstructionAction::Subtract,
                _ => return Err(InstructionParseError),
            },
            None => return Err(InstructionParseError),
        };

        let focal = match action {
            InstructionAction::Append => match caps.name("focal") {
                Some(a) => Some(a.as_str().parse::<u8>().unwrap()),
                None => return Err(InstructionParseError),
            },
            InstructionAction::Subtract => None,
        };

        Ok(Instruction {
            action,
            focal,
            label,
        })
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal: u8,
}
