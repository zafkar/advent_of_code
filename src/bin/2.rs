use advent_of_code::load_data;
use std::io::BufRead;
use std::str::FromStr;

const ADVENT_NUM: &str = "2";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let mut all_games = vec![];
    for line in file.lines().map(|a| a.unwrap()) {
        println!("=> {line}");
        let game: Game = line.parse().unwrap();
        all_games.push(game);
        println!(
            "{:?}, score => {}",
            all_games.last().unwrap(),
            all_games.last().unwrap().get_total()
        );
    }

    let total: u32 = all_games
        .iter()
        .filter(|g| !(g.is_ge("red", 12) || g.is_ge("green", 13) || g.is_ge("blue", 14)))
        .map(|g| g.0)
        .sum();

    println!("Total => {total}");

    let sum_power: u32 = all_games.iter().map(|g| g.get_total()).sum();
    println!("SumPower => {sum_power}");
}

#[derive(Debug)]
struct Pull(String, u32);

#[derive(Debug)]
struct ParseErr;

impl FromStr for Pull {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp: Vec<&str> = s.split_whitespace().collect();
        let color = match tmp.last() {
            Some(a) => a.to_string(),
            _ => return Err(ParseErr),
        };
        let q: u32 = match tmp.first() {
            Some(a) => match a.parse() {
                Ok(b) => b,
                _ => return Err(ParseErr),
            },
            _ => return Err(ParseErr),
        };
        Ok(Pull(color, q))
    }
}

#[derive(Debug)]
struct Reach(Vec<Pull>);

impl FromStr for Reach {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pulls: Vec<Pull> = match s.split(',').map(|p| p.parse()).collect() {
            Ok(a) => a,
            _ => return Err(ParseErr),
        };
        Ok(Reach(pulls))
    }
}

impl Reach {
    fn is_ge(&self, color: &str, val: u32) -> bool {
        for p in self.0.iter() {
            if p.0 == color {
                return p.1 > val;
            }
        }
        false
    }

    fn get(&self, color: &str) -> u32 {
        for p in self.0.iter() {
            if p.0 == color {
                return p.1;
            }
        }
        0
    }
}

#[derive(Debug)]
struct Game(u32, Vec<Reach>);

impl FromStr for Game {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, game_data)) = s.split_once(':') {
            let reaches: Vec<Reach> = match game_data.split(';').map(|a| a.parse()).collect() {
                Ok(a) => a,
                _ => return Err(ParseErr),
            };
            let game_num: u32 = match label[5..].parse() {
                Ok(a) => a,
                _ => return Err(ParseErr),
            };
            return Ok(Game(game_num, reaches));
        }

        Err(ParseErr)
    }
}

impl Game {
    fn is_ge(&self, color: &str, val: u32) -> bool {
        for r in self.1.iter() {
            if r.is_ge(color, val) {
                return true;
            }
        }
        false
    }

    fn max(&self, color: &str) -> u32 {
        self.1
            .iter()
            .map(|r| r.get(color))
            .fold(0, |a, v| if v > a { v } else { a })
    }

    fn get_total(&self) -> u32 {
        let red = self.max("red");
        let green = self.max("green");
        let blue = self.max("blue");
        red * green * blue
    }
}
