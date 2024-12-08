use advent_of_code::{load_data, Direction, Point};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{error::Error, fmt::Display, io::Read, str::FromStr};

const ADVENT_NUM: &str = "2024/6";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let mut grid: Grid = full_text.parse()?;
    let part2_grid = grid.clone();
    println!("{}", grid);
    while grid.update() {
        //sleep(Duration::from_millis(500));
        grid.update();
        //println!("{}\n\n", grid);
    }
    println!("{}", grid);
    println!("part1 => {}", grid.count_visited());

    println!("part2 => {}", count_loop_possibilities(&part2_grid));
    Ok(())
}

fn count_loop_possibilities(original_grid: &Grid) -> usize {
    let grid_size = original_grid.grid.len();
    let all_possibilities_pos: Vec<Point> = (0..grid_size)
        .flat_map(|x| {
            (0..grid_size).map(move |y| Point {
                x: x as i64,
                y: y as i64,
            })
        })
        .collect_vec();
    all_possibilities_pos
        .par_iter()
        .filter(|pos| original_grid.check_loop_possiblities(**pos))
        .count()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Guard {
    pos: Point,
    direction: Direction,
}

impl Guard {
    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => panic!("Should never be here"),
        }
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dis_char = match self.direction {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
            _ => panic!("Should never be here"),
        };
        write!(f, "{}", dis_char)?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileState {
    Obstructed,
    Empty,
    Visited,
}

impl Display for TileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dis_char = match self {
            TileState::Empty => '.',
            TileState::Obstructed => '#',
            TileState::Visited => 'X',
        };
        write!(f, "{}", dis_char)?;
        Ok(())
    }
}

#[derive(Clone)]
struct Grid {
    guard: Guard,
    grid: Vec<Vec<TileState>>,
}

impl Grid {
    fn update(&mut self) -> bool {
        self.set_tile(self.guard.pos, TileState::Visited);
        let new_guard_pos = self.guard.pos + self.guard.direction;
        match self.get_tile(new_guard_pos) {
            Some(TileState::Empty) | Some(TileState::Visited) => self.guard.pos = new_guard_pos,
            Some(TileState::Obstructed) => self.guard.rotate_right(),
            None => return false,
        };
        true
    }

    fn get_tile(&self, pos: Point) -> Option<&TileState> {
        if let Some(row) = self.grid.get(pos.y as usize) {
            return row.get(pos.x as usize);
        }
        None
    }

    fn set_tile(&mut self, pos: Point, tile: TileState) {
        self.grid[pos.y as usize][pos.x as usize] = tile;
    }

    fn count_visited(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|tile| **tile == TileState::Visited)
            .count()
    }

    fn check_loop_possiblities(&self, pos: Point) -> bool {
        println!("Checking {pos:?}");
        let mut current_grid = self.clone();
        current_grid.set_tile(pos, TileState::Obstructed);
        let mut guard_history = vec![];
        while current_grid.update() {
            if guard_history.contains(&current_grid.guard) {
                return true;
            }
            guard_history.push(current_grid.guard.clone());
        }
        false
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, state) in line.iter().enumerate() {
                if self.guard.pos == (x, y) {
                    write!(f, "{}", self.guard)?;
                } else {
                    write!(f, "{}", state)?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = GenericParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        let mut guard_pos: Point = (0, 0).into();
        for (y, line) in s.lines().enumerate() {
            let mut current_row = vec![];
            for (x, c) in line.char_indices() {
                let tile = match c {
                    '#' => TileState::Obstructed,
                    '.' => TileState::Empty,
                    '^' => {
                        guard_pos = (x, y).into();
                        TileState::Empty
                    }
                    _ => return Err(GenericParseError("Unknown char".to_string())),
                };
                current_row.push(tile);
            }
            grid.push(current_row);
        }

        let guard = Guard {
            pos: guard_pos,
            direction: Direction::North,
        };

        Ok(Grid { guard, grid })
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
