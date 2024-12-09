use advent_of_code::{load_data, AdventError};
use std::{error::Error, fmt::Display, io::BufRead, ops::Mul, str::FromStr};

const ADVENT_NUM: &str = "2024/9";

fn main() -> Result<(), Box<dyn Error>> {
    let file = load_data(ADVENT_NUM, "input.txt")?;
    for line in file.lines() {
        let mut row: Disk = line?.parse()?;
        let mut part2 = row.clone();
        println!("{row}");
        row.compact();
        println!("Part 1 => {}", row.checksum());

        part2.defrag();
        println!("Part 2 => {}", part2.checksum());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskState {
    Empty,
    File(u32),
}

#[derive(Debug, Clone, Copy)]
struct IndexEntry {
    content: DiskState,
    start: usize,
    len: usize,
}
impl IndexEntry {
    fn new(content: DiskState, start: usize, len: usize) -> IndexEntry {
        IndexEntry {
            content,
            start,
            len,
        }
    }
}

#[derive(Debug, Clone)]
struct Disk(Vec<DiskState>);

impl Disk {
    fn compact(&mut self) {
        let mut end_pointer = self.0.len() - 1;
        for index in 0..self.0.len() {
            if index >= end_pointer - 1 {
                break;
            }
            if let Some(DiskState::File(_)) = self.0.get(index) {
                continue;
            }
            while let Some(DiskState::Empty) = self.0.get(end_pointer) {
                end_pointer -= 1;
            }

            self.0.swap(index, end_pointer);
        }
    }

    fn move_blocks(&mut self, src: usize, dst: usize, len: usize) {
        println!("Moving {src} to {dst} for {len}");
        for i in 0..len {
            self.0.swap(src + i, dst + i);
        }
    }

    fn build_block_index_until(&self, end: usize) -> Vec<IndexEntry> {
        let mut result = vec![];
        let mut last_block = DiskState::Empty;
        let mut current_len = 0;
        let mut current_start = 0;
        for (index, current_cell) in self.0[0..end].iter().enumerate() {
            if *current_cell != last_block {
                result.push(IndexEntry::new(last_block, current_start, current_len));
                current_start = index;
                current_len = 0;
            }

            current_len += 1;
            last_block = *current_cell;
        }
        result.push(IndexEntry::new(last_block, current_start, current_len));
        result.remove(0);
        result
    }

    fn defrag(&mut self) {
        let mut current_end = self.0.len();
        'outer: while current_end > 0 {
            let disk_index = self.build_block_index_until(current_end);
            for (last_possible_block_index, block) in disk_index
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, e)| e.content != DiskState::Empty)
            {
                for empty_space in disk_index[0..last_possible_block_index]
                    .iter()
                    .filter(|e| e.content == DiskState::Empty)
                {
                    if empty_space.len >= block.len {
                        self.move_blocks(block.start, empty_space.start, block.len);
                        current_end = block.start;
                        continue 'outer;
                    }
                }
            }
            break;
        }
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(index, cell)| (*cell * index) as u64)
            .sum()
    }
}

impl FromStr for Disk {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];
        let mut file_id = 0;
        let mut is_file = true;
        for num in s.chars() {
            let cell_content = match is_file {
                true => DiskState::File(file_id),
                false => DiskState::Empty,
            };
            let block_len = num.to_string().parse::<usize>().unwrap();
            for _ in 0..block_len {
                v.push(cell_content);
            }
            if is_file {
                file_id += 1;
            }
            is_file = !is_file;
        }
        Ok(Disk(v))
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in self.0.iter() {
            write!(f, "|{cell}")?;
        }
        Ok(())
    }
}

impl Display for DiskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            DiskState::Empty => '.'.to_string(),
            DiskState::File(a) => a.to_string(),
        };
        write!(f, "{c}")
    }
}

impl Mul<usize> for DiskState {
    type Output = usize;
    fn mul(self, rhs: usize) -> Self::Output {
        if let DiskState::File(a) = self {
            a as usize * rhs
        } else {
            0
        }
    }
}
