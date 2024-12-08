use std::{
    fs::File,
    io::{BufReader, Error},
};

mod direction;
mod point;

pub use direction::Direction;
pub use point::Point;

pub fn load_data(advent_num: &str, filename: &str) -> Result<BufReader<File>, Error> {
    let path = format!("data/{advent_num}/{filename}");
    Ok(BufReader::new(File::open(path)?))
}
