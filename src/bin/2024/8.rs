use advent_of_code::{load_data, Point};
use std::{collections::HashMap, error::Error, fmt::Display, io::Read, str::FromStr};

const ADVENT_NUM: &str = "2024/8";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let mut grid: Grid = full_text.parse()?;

    println!("{}", grid);

    println!();

    for pair in grid.get_all_antenna_pairs() {
        println!("{:?}", pair);
    }

    grid.update_antinodes();

    println!("{}", grid);

    println!("Part 1 => {}", grid.count_antinode());

    Ok(())
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
            return ;
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
            let mut mult = 0;
            while self.is_within_bound(left + distance * mult) {
                self.set_tile(left + distance * mult, TileState::Antinode);
                mult += 1;
            }
            mult = -1;
            while self.is_within_bound(left + distance * mult) {
                self.set_tile(left + distance * mult, TileState::Antinode);
                mult -= 1;
            }
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
            writeln!(f)?;
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
                        antennas.entry(a).or_default().push((x, y).into());
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
