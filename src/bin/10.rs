use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let maze = Maze::from(file.lines().map(|f| f.unwrap()).collect::<Vec<String>>());
    println!("Furthest Point => {}", maze.find_size_loop());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    StartingPos,
}

struct Maze {
    pos: Pos,
    old_pos: Pos,
    start_pos: Pos,
    maze: Vec<Vec<TileType>>,
}

impl From<Vec<String>> for Maze {
    fn from(value: Vec<String>) -> Self {
        let mut maze = vec![];
        let mut start_pos = Pos { x: 0, y: 0 };
        for (y_index, line) in value.iter().enumerate() {
            let mut row = vec![];
            for (x_index, c) in line.char_indices() {
                row.push(match c {
                    '|' => TileType::Vertical,
                    '-' => TileType::Horizontal,
                    'J' => TileType::BendNW,
                    '7' => TileType::BendSW,
                    'F' => TileType::BendSE,
                    'L' => TileType::BendNE,
                    '.' => TileType::Ground,
                    'S' => {
                        start_pos = Pos {
                            x: x_index as i32,
                            y: y_index as i32,
                        };
                        TileType::StartingPos
                    }
                    _ => unimplemented!("Couldn't recognize char {c}"),
                })
            }
            maze.push(row);
        }
        Maze {
            pos: start_pos,
            old_pos: start_pos,
            start_pos,
            maze,
        }
    }
}

const ADJACENCY: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Maze {
    fn get(&self, x: i32, y: i32) -> Option<&TileType> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(row) = self.maze.get(y as usize) {
            row.get(x as usize)
        } else {
            None
        }
    }

    fn get_pos(&self, pos: Pos) -> Option<&TileType> {
        self.get(pos.x, pos.y)
    }

    fn half_match(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> bool {
        let tile_1 = match self.get(x_1, y_1) {
            Some(a) => a,
            _ => return false,
        };
        let tile_2 = match self.get(x_2, y_2) {
            Some(a) => a,
            _ => return false,
        };
        //println!("{tile_1:?} at {x_1}, {y_1} and {tile_2:?} at {x_2}, {y_2}");
        match (
            tile_1,
            tile_2,
            x_1 as i32 - x_2 as i32,
            y_1 as i32 - y_2 as i32,
        ) {
            (TileType::StartingPos, TileType::Horizontal, a, 0) if a == 1 || a == -1 => true,
            (TileType::StartingPos, TileType::Vertical, 0, a) if a == 1 || a == -1 => true,
            (TileType::StartingPos, TileType::BendNE, 1, 0) => true,
            (TileType::StartingPos, TileType::BendNE, 0, -1) => true,
            (TileType::StartingPos, TileType::BendNW, -1, 0) => true,
            (TileType::StartingPos, TileType::BendNW, 0, -1) => true,
            (TileType::StartingPos, TileType::BendSW, -1, 0) => true,
            (TileType::StartingPos, TileType::BendSW, 0, 1) => true,
            (TileType::StartingPos, TileType::BendSE, 1, 0) => true,
            (TileType::StartingPos, TileType::BendSE, 0, 1) => true,
            (TileType::Horizontal, TileType::Horizontal, a, 0) if a == -1 || a == 1 => true,
            (TileType::Horizontal, TileType::BendSE, 1, 0) => true,
            (TileType::Horizontal, TileType::BendNE, 1, 0) => true,
            (TileType::Horizontal, TileType::BendSW, -1, 0) => true,
            (TileType::Horizontal, TileType::BendNW, -1, 0) => true,
            (TileType::Vertical, TileType::Vertical, 0, a) if a == -1 || a == 1 => true,
            (TileType::Vertical, TileType::BendNW, 0, -1) => true,
            (TileType::Vertical, TileType::BendNE, 0, -1) => true,
            (TileType::Vertical, TileType::BendSW, 0, 1) => true,
            (TileType::Vertical, TileType::BendSE, 0, 1) => true,
            (TileType::BendNE, TileType::BendNW, -1, 0) => true,
            (TileType::BendNE, TileType::BendSE, 0, 1) => true,
            (TileType::BendNE, TileType::BendSW, -1, 0) => true,
            (TileType::BendNE, TileType::BendSW, 0, 1) => true,
            (TileType::BendSW, TileType::BendNW, 0, -1) => true,
            (TileType::BendSW, TileType::BendSE, 1, 0) => true,
            (TileType::BendNW, TileType::BendSE, 0, 1) => true,
            (TileType::BendNW, TileType::BendSE, 1, 0) => true,
            (tile_1, tile_2, x_diff, y_diff) => {
                #[cfg(debug_assertions)]
                println!("{tile_1:?} and {tile_2:?} not connected {x_diff},{y_diff}");
                false
            }
        }
    }

    fn is_connected(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> bool {
        let sym_1 = self.half_match(x_1, y_1, x_2, y_2);
        if sym_1 {
            return true;
        }
        self.half_match(x_2, y_2, x_1, y_1)
    }

    fn find_connexion(&self, pos: Pos) -> (Pos, Pos) {
        let mut found = vec![];
        for (x, y) in ADJACENCY.iter() {
            if self.is_connected(pos.x, pos.y, pos.x + *x, pos.y + *y) {
                found.push(Pos {
                    x: pos.x + *x,
                    y: pos.y + *y,
                });
            }
        }

        (*found.get(0).unwrap(), *found.get(1).unwrap())
    }

    fn step(&mut self) -> () {
        let possible_pos = self.find_connexion(self.pos);
        let new_pos = match self.old_pos {
            a if a == self.pos => possible_pos.0,
            a if a == possible_pos.0 => possible_pos.1,
            a if a == possible_pos.1 => possible_pos.0,
            _ => unimplemented!("Should never be here"),
        };
        self.old_pos = self.pos;
        self.pos = new_pos;
        println!("Moved to {new_pos:?}");
    }

    fn find_size_loop(&self) -> u32 {
        let mut c = Maze {
            pos: self.start_pos,
            old_pos: self.start_pos,
            start_pos: self.start_pos,
            maze: self.maze.clone(),
        };
        let mut size = 1;
        c.display_current_tile();
        c.step();
        while *c.get_pos(c.pos).unwrap() != TileType::StartingPos {
            c.display_current_tile();
            c.step();
            size += 1;
        }
        size / 2 + size % 2
    }

    fn display_current_tile(&self) -> () {
        println!("Current tile {:?}", self.get_pos(self.pos).unwrap());
    }
}
