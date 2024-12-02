#![allow(clippy::needless_return)]

use std::{error::Error,path::PathBuf};

mod part1;
use part1::part1;
mod part2;
use part2::part2;

fn main() -> Result<(), Box<dyn Error>> {
    let current_file = file!();
    let current_dir = PathBuf::from(current_file);

    let prefix = current_dir.parent().expect("Something went terribly wrong...");

    let part1_result = part1(prefix.join("input.dat").to_str().expect("File not found!"))?;
    println!("Answer to part1: {}",part1_result);
    let part2_result = part2(prefix.join("input.dat").to_str().expect("File not found!"))?;
    println!("Answer to part2: {}", part2_result);

    return Ok(());
}
