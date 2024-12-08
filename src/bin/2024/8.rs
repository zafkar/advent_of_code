use advent_of_code::load_data;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::Read,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

const ADVENT_NUM: &str = "2024/8";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let mut grid: Grid = full_text.parse()?;

    println!("{}", grid);

    println!("");

    for pair in grid.get_all_antenna_pairs() {
        println!("{:?}", pair);
    }

    grid.update_antinodes();

    println!("{}", grid);

    println!("Part 1 => {}", grid.count_antinode());

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 as i64 && self.y == other.1 as i64
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileState {
    Antinode,
    Empty,
}

impl Display for TileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dis_char = match self {
            TileState::Empty => '.',
            TileState::Antinode => 'X',
        };
        write!(f, "{}", dis_char)?;
        Ok(())
    }
}

#[derive(Clone)]
struct Grid {
    antennas: HashMap<char, Vec<Point>>,
    grid: Vec<Vec<TileState>>,
}

impl Grid {
    fn _get_tile(&self, pos: Point) -> Option<&TileState> {
        if let Some(row) = self.grid.get(pos.y as usize) {
            return row.get(pos.x as usize);
        }
        None
    }

    fn width(&self) -> usize {
        self.grid.len()
    }

    fn height(&self) -> usize {
        self.width()
    }

    fn is_within_bound(&self, pos: Point) -> bool {
        !(pos.x >= self.width() as i64 || pos.x < 0 || pos.y < 0 || pos.y >= self.height() as i64)
    }

    fn set_tile(&mut self, pos: Point, tile: TileState) {
        if pos.x >= self.width() as i64 || pos.x < 0 || pos.y < 0 || pos.y >= self.height() as i64 {
            return ();
        }
        self.grid[pos.y as usize][pos.x as usize] = tile;
    }

    fn get_all_antenna_pairs(&self) -> impl Iterator<Item = (char, Point, Point)> + '_ {
        self.antennas.keys().flat_map(move |freq| {
            self.antennas
                .get(freq)
                .unwrap()
                .iter()
                .enumerate()
                .flat_map(move |(index, left_antenna)| {
                    self.antennas.get(freq).unwrap()[index + 1..]
                        .iter()
                        .map(move |right_antenna| (*freq, *left_antenna, *right_antenna))
                })
        })
    }

    fn update_antinodes(&mut self) {
        for (_, left, right) in self.clone().get_all_antenna_pairs() {
            let distance = left - right;
            self.set_tile(right - distance, TileState::Antinode);
            self.set_tile(left + distance, TileState::Antinode);
        }
    }

    fn count_antinode(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|tile| **tile == TileState::Antinode)
            .count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_y, line) in self.grid.iter().enumerate() {
            for (_x, state) in line.iter().enumerate() {
                write!(f, "{}", state)?;
            }
            writeln!(f, "")?;
        }
        for (freq, antennas) in self.antennas.iter() {
            for antenna in antennas.iter() {
                writeln!(f, "{} at {:?}", freq, antenna)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            let mut current_row = vec![];
            for (x, c) in line.char_indices() {
                let tile = match c {
                    '.' => TileState::Empty,
                    a => {
                        antennas.entry(a).or_insert(vec![]).push((x, y).into());
                        TileState::Empty
                    }
                };
                current_row.push(tile);
            }
            grid.push(current_row);
        }

        Ok(Grid { grid, antennas })
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
