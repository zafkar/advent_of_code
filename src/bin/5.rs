use advent_of_code::load_data;
use std::{io::BufRead, str::FromStr, sync::mpsc::channel, thread::spawn};

const ADVENT_NUM: &str = "5";

fn main() {
    let mut file = load_data(ADVENT_NUM, "input.txt").unwrap();
    let mut _seeds: Vec<u64> = vec![];

    let mut line_buffer = String::new();
    file.read_line(&mut line_buffer).unwrap();
    let (_, seeds_txt) = line_buffer.split_once(':').unwrap();
    _seeds = seeds_txt
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    file.read_line(&mut line_buffer).unwrap();

    let mut all_maps = vec![];
    let mut current_map: Vec<String> = vec![];
    for line in file.lines().map(|l| l.unwrap()) {
        println!("{line}");
        if line.is_empty() {
            all_maps.push(Mapping::from(current_map.clone()));
            current_map = vec![];
        } else {
            current_map.push(line);
        }
    }

    for m in all_maps.iter() {
        println!("{m:?}");
    }

    println!("Finding seeds\n ################ \n");

    let (tx, rx) = channel();
    let mut threads = vec![];
    for (index, seed_pair) in _seeds.chunks_exact(2).enumerate() {
        let (seed_start, seed_len) = match seed_pair {
            [seed, seed_len] => (*seed, *seed_len),
            _ => panic!("Error while depariring seed input"),
        };
        println!("\n{index}/{}", _seeds.len() / 2);
        let all_maps_clone = all_maps.clone();
        let t_tx = tx.clone();
        threads.push(spawn(move || {
            for (seed_index, seed) in (seed_start..(seed_start + seed_len)).enumerate() {
                if seed_index % 1000000 == 0 {
                    println!("Slot {index} => {}%", seed_index as u64 * 100 / seed_len);
                }
                let mut current = seed;
                for m in all_maps_clone.iter() {
                    let prev = current;
                    current = m.convert(&current);
                    #[cfg(debug_assertions)]
                    println!("{prev} to {current} via {}2{}", m.src_label, m.dest_label);
                }
                //println!("\t{seed} is to be planted in {current}");
                t_tx.send(current).unwrap();
            }
            println!("\tSlot {index} finished !");
        }));
    }

    drop(tx);

    let min_loc = rx.iter().fold(
        u64::max_value(),
        |acc, loc| if loc < acc { loc } else { acc },
    );
    println!("Min_loc => {min_loc}");
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Clone, Copy)]
struct Range {
    src: u64,
    dest: u64,
    len: u64,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [dest, src, len] = s.split_whitespace().collect::<Vec<_>>()[..3] {
            Ok(Range {
                src: src.parse().unwrap(),
                dest: dest.parse().unwrap(),
                len: len.parse().unwrap(),
            })
        } else {
            Err(ParseError)
        }
    }
}

impl Range {
    fn convert(&self, src: &u64) -> Option<u64> {
        if (self.src..(self.len + self.src)).contains(src) {
            Some(self.dest + src - self.src)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    src_label: String,
    dest_label: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn convert(&self, src: &u64) -> u64 {
        for r in self.ranges.iter() {
            if let Some(result) = r.convert(src) {
                return result;
            }
        }
        *src
    }
}

impl From<Vec<String>> for Mapping {
    fn from(value: Vec<String>) -> Self {
        let header = value.first().unwrap();
        let [src_label, dest_label] = match header[..header.len() - 5]
            .split("-to-")
            .collect::<Vec<&str>>()[..2]
        {
            [a, b] => [a.to_string(), b.to_string()],
            _ => panic!("wrong header"),
        };

        let mut ranges = vec![];
        for line in value[1..].iter() {
            ranges.push(line.parse().unwrap());
        }

        Mapping {
            src_label,
            dest_label,
            ranges,
        }
    }
}
