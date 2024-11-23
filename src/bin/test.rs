use std::error::Error;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hellow_world");
    for comb in (0..5).combinations(3) {
        println!("{comb:?}");
    }
    Ok(())
}
