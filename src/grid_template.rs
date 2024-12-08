fn main() {
    let mut full_text = String::new();
    file.read_to_string(&mut full_text)?;

    let mut grid: Grid = full_text.parse()?;
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
    grid: Vec<Vec<TileState>>,
}

impl Grid {
    fn get_tile(&self, pos: Point) -> Option<&TileState> {
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

    fn is_within_bound(&self, pos: Point) -> bool {
        !(pos.x >= self.width() as i64 || pos.x < 0 || pos.y < 0 || pos.y >= self.height() as i64)
    }

    fn set_tile(&mut self, pos: Point, tile: TileState) {
        if pos.x >= self.width() as i64 || pos.x < 0 || pos.y < 0 || pos.y >= self.height() as i64 {
            return ();
        }
        self.grid[pos.y as usize][pos.x as usize] = tile;
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
