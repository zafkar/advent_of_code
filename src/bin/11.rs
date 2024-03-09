use advent_of_code::load_data;
use std::{
    fmt::Display,
    io::BufRead,
    sync::mpsc::{channel, Sender},
};

const ADVENT_NUM: &str = "11";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let space = Space(file.lines().map(|f| f.unwrap()).collect());

    let gs = space.get_galaxies();
    println!("{gs:?}",);
    println!("Search for all distances");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let (tx, rx) = channel();
    for (i, p) in gs.iter().enumerate() {
        let tx_clone: Sender<u64> = tx.clone();
        let p_clone = p.clone();
        let space_clone = space.clone();
        let work: Vec<Pos> = gs[i + 1..].iter().map(|p| *p).collect();
        pool.spawn(move || {
            for o in work {
                tx_clone
                    .send(space_clone.get_actual_dist(p_clone, o, 1_000_000))
                    .unwrap();
            }
            println!("Slot {i} completed")
        });
    }
    drop(tx);

    let acc = rx.iter().fold(0, |acc, v| acc + v);

    println!("Sum of shirtest path {acc}");
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn _get_dist(&self, other: Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
struct Space(Vec<String>);

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|s| {
            writeln!(f, "{s}").unwrap();
        });
        Ok(())
    }
}

impl Space {
    fn get_galaxies(&self) -> Vec<Pos> {
        let mut galaxies = vec![];
        for (y, line) in self.0.iter().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => galaxies.push(Pos { x, y }),
                    _ => (),
                }
            }
        }

        galaxies
    }

    fn get_actual_dist(&self, a: Pos, b: Pos, n: u64) -> u64 {
        let (left_most, right_most) = if a.x > b.x { (b.x, a.x) } else { (a.x, b.x) };
        let (top_most, bottom_most) = if a.y > b.y { (b.y, a.y) } else { (a.y, b.y) };
        let mut dist = 0;
        for x in left_most..right_most {
            dist += 1;
            if self.is_column_empty(x) {
                dist += n - 1;
            }
        }
        for y in top_most..bottom_most {
            dist += 1;
            if self.is_row_empty(y) {
                dist += n - 1;
            }
        }
        dist
    }

    fn _get_expanded(&self, n: usize) -> Space {
        let height = self.get_height();
        let width = self._get_width();
        let mut expanded_space: Vec<String> = vec![];
        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                line.push(self.get_index(x, y).unwrap());
                if self.is_column_empty(x) {
                    for _ in 1..n {
                        line.push('.');
                    }
                }
            }
            expanded_space.push(line.clone());
            if self.is_row_empty(y) {
                for _ in 1..n {
                    expanded_space.push(line.clone());
                }
            }
        }
        Space(expanded_space)
    }

    fn get_index(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.0.get(y) {
            row.chars().nth(x)
        } else {
            None
        }
    }

    fn _get_width(&self) -> usize {
        self.0.get(0).unwrap().len()
    }

    fn get_height(&self) -> usize {
        self.0.len()
    }

    fn column(&self, x: usize) -> String {
        (0..self.get_height())
            .map(|y| self.get_index(x, y))
            .map(|c| c.unwrap())
            .collect()
    }

    fn is_row_empty(&self, y: usize) -> bool {
        !self.0.get(y).unwrap().contains('#')
    }

    fn is_column_empty(&self, x: usize) -> bool {
        !self.column(x).contains('#')
    }
}
