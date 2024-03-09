use std::{
    fs::File,
    io::{BufReader, Error},
};

pub fn load_data(advent_num: &str, filename: &str) -> Result<BufReader<File>, Error> {
    let path = format!("data/{advent_num}/{filename}");
    Ok(BufReader::new(File::open(path)?))
}
