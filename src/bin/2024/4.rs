use std::{error::Error, io::Read};

use advent_of_code::load_data;

const ADVENT_NUM: &str = "2024/4";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = load_data(ADVENT_NUM, "input.txt")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let grid = text.lines().map(|s| s.to_string()).collect::<Vec<String>>();

    let result_part1 = (0..grid.len())
        .flat_map(|x| (0..grid.len()).map(move |y| (x, y)))
        .map(|pos| check_word(&grid, pos.0 as i32, pos.1 as i32))
        .sum::<u32>();
    println!("Part1 => {result_part1:?}");

    let result_part2 = (0..grid.len())
        .flat_map(|x| (0..grid.len()).map(move |y| (x, y)))
        .filter(|pos| check_cross(&grid, pos.0 as i32, pos.1 as i32))
        .count();
    println!("Part2 => {result_part2:?}");

    Ok(())
}

fn get_char(grid: &Vec<String>, x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }

    let row = match grid.get(y as usize) {
        Some(r) => r,
        None => return None,
    };
    row.chars().nth(x as usize)
}

fn check_word(grid: &Vec<String>, x: i32, y: i32) -> u32 {
    let word = "XMAS";
    let mut current_directions = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    for (word_index, word_c) in word.chars().enumerate() {
        let mut new_directions = vec![];
        for (x_offset, y_offset) in current_directions {
            match get_char(
                grid,
                x + x_offset * word_index as i32,
                y + y_offset * word_index as i32,
            ) {
                Some(c) if c == word_c => new_directions.push((x_offset, y_offset)),
                _ => (),
            }
        }
        if new_directions.len() == 0 {
            return 0;
        }
        current_directions = new_directions.clone();
    }
    current_directions.len() as u32
}

fn check_cross(grid: &Vec<String>, x: i32, y: i32) -> bool {
    if get_char(grid, x, y) != Some('A') {
        return false;
    }

    let dirs_to_check = vec![
        ((-1, -1), (1, 1)),
        ((1, 1), (-1, -1)),
        ((-1, 1), (1, -1)),
        ((1, -1), (-1, 1)),
    ];
    let mut crossed = 0;
    for (left, right) in dirs_to_check {
        if get_char(grid, x + left.0, y + left.1) == Some('M')
            && get_char(grid, x + right.0, y + right.1) == Some('S')
        {
            crossed += 1;
        }
    }
    crossed >= 2
}
