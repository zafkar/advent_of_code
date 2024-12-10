use advent_of_code::{load_data, AdventError, Direction, Point};
use itertools::Itertools;
use std::{error::Error, fmt::Display, io::Read, str::FromStr};

const ADVENT_NUM: &str = "2024/10";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let grid: Grid = full_text.parse()?;
    println!("{}", grid);

    let part1: usize = grid
        .get_all_trailhead()
        .iter()
        .map(|pos| grid.count_trail_score(*pos))
        .sum();
    println!("part1 => {part1}",);

    let part2: usize = grid
        .get_all_trailhead()
        .iter()
        .map(|pos| grid.count_trail_rating(*pos))
        .sum();
    println!("part2 => {part2}");

    Ok(())
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn get_surrounding(&self, pos: Point) -> Vec<(Point, Option<&u8>)> {
        Direction::get_ortho()
            .iter()
            .map(|dir| (pos + *dir, self.get_tile(pos + *dir)))
            .collect_vec()
    }

    fn get_all_trails(&self, start_pos: Point) -> Vec<Vec<Point>> {
        let mut all_current_path = vec![vec![start_pos]];
        for current_height in 1..10 {
            let mut all_next_path = vec![];
            for path in all_current_path.iter() {
                if let Some(last_cell) = path.last() {
                    for next_cells in self.get_surrounding(*last_cell) {
                        if let Some(h) = next_cells.1 {
                            if *h == current_height {
                                let mut new_path = path.clone();
                                new_path.push(next_cells.0);
                                all_next_path.push(new_path);
                            }
                        }
                    }
                }
            }
            all_current_path = all_next_path;
        }
        all_current_path
    }

    fn count_trail_score(&self, start_pos: Point) -> usize {
        self.get_all_trails(start_pos)
            .iter()
            .map(|path| *path.last().unwrap())
            .unique()
            .count()
    }

    fn count_trail_rating(&self, start_pos: Point) -> usize {
        self.get_all_trails(start_pos).len()
    }

    fn get_all_trailhead(&self) -> Vec<Point> {
        (0..self.width())
            .flat_map(|x| (0..self.height()).map(move |y| Point::from((x, y))))
            .filter(|pos| *self.get_tile(*pos).unwrap() == 0)
            .collect_vec()
    }

    fn get_tile(&self, pos: Point) -> Option<&u8> {
        if let Some(row) = self.grid.get(pos.y as usize) {
            return row.get(pos.x as usize);
        }
        None
    }

    fn width(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.grid.len()
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
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for (_y, line) in s.lines().enumerate() {
            let mut current_row = vec![];
            for (_x, c) in line.char_indices() {
                let tile = match c.to_string().parse() {
                    Ok(a) => a,
                    Err(e) => {
                        return Err(AdventError::new(&format!("Couldn't parse grid : {}", e)))
                    }
                };
                current_row.push(tile);
            }
            grid.push(current_row);
        }

        Ok(Grid { grid })
    }
}
