use advent_of_code::load_data;
use itertools::{max, Itertools};
use std::{error::Error, fmt::Display, io::Read, iter::zip, str::FromStr};

const ADVENT_NUM: &str = "20228";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut text_log = String::new();
    file.read_to_string(&mut text_log)?;

    let grid: Grid = text_log.parse()?;

    println!("{grid}");
    let result = (0..grid.0)
        .flat_map(|x| (0..grid.1).map(move |y| (x, y)))
        .map(|(x, y)| grid.score(x, y))
        .max()
        .unwrap();

    println!("{result}");

    Ok(())
}

#[derive(Debug)]
struct Grid(usize, usize, Vec<i8>);

impl Grid {
    fn get(&self, x: usize, y: usize) -> i8 {
        match self.2.get(x + y * self.0) {
            Some(a) => *a,
            None => -1,
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.0 - 1 || y == self.1 - 1 {
            return true;
        }

        let target_size = self.get(x, y);
        let max_east = (x + 1..self.0).map(|i| self.get(i, y)).max().unwrap_or(-1);
        let max_west = (0..x).map(|i| self.get(i, y)).max().unwrap_or(-1);
        let max_south = (y + 1..self.1).map(|i| self.get(x, i)).max().unwrap_or(-1);
        let max_north = (0..y).map(|i| self.get(x, i)).max().unwrap_or(-1);

        //println!("({x},{y}) => E({max_east}), N({max_north}), W({max_west}), S({max_south})");
        &target_size
            > vec![max_east, max_west, max_south, max_north]
                .iter()
                .min()
                .unwrap_or(&-1)
    }

    fn score(&self, x: usize, y: usize) -> u32 {
        let target_size = self.get(x, y);

        let score_east = (x + 1..self.0)
            .map(|i| self.get(i, y))
            .take_while_inclusive(|h| target_size > *h)
            .count();
        let score_west = (0..x)
            .rev()
            .map(|i| self.get(i, y))
            .take_while_inclusive(|h| target_size > *h)
            .count();
        let score_south = (y + 1..self.1)
            .map(|i| self.get(x, i))
            .take_while_inclusive(|h| target_size > *h)
            .count();
        let score_north = (0..y)
            .rev()
            .map(|i| self.get(x, i))
            .take_while_inclusive(|h| target_size > *h)
            .count();
        // println!(
        //     "({x},{y}) => E({score_east}), N({score_north}), W({score_west}), S({score_south})"
        // );
        (score_east * score_north * score_south * score_west)
            .try_into()
            .unwrap_or(0)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.1 {
            for x in 0..self.0 {
                write!(f, "{:3}|", self.score(x, y))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = GenericParseError;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        let lines = i.lines().collect_vec();
        let width = lines.first().unwrap_or(&"").len();
        let height = lines.len();
        let grid = i
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| String::from(c))
            .map(|s| s.parse::<i8>().unwrap())
            .collect();

        Ok(Grid(width, height, grid))
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
