#![allow(clippy::needless_return)]

use std::error::Error;

mod part1;
use part1::part1;
mod part2;
use part2::part2;

fn main() -> Result<(), Box<dyn Error>> {
    let part1_path = "src/day3/part1.dat";

    let part1_result = part1(part1_path)?;
    println!("Answer to part1: {}",part1_result);

    let part2_path = "src/day3/part1.dat";
    let part2_result = part2(part2_path)?;
    println!("Answer to part2: {}", part2_result);

    return Ok(());
}
