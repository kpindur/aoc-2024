use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};


type Int = i32;

pub fn load_data(path: &str) -> Result<(Vec<Int>, Vec<Int>), Box<dyn Error>> {
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let (left, right): (Vec<Int>, Vec<Int>) = reader.lines()
                        .map(|line| line.unwrap())
                        .map(|line| {
                            let mut parts = line.split_whitespace();
                            (
                                parts.next().unwrap().parse::<Int>().expect("Failed at parsing value!"),
                                parts.next().unwrap().parse::<Int>().expect("Failed at parsing value!")
                            )
                        })
                        .unzip();

    return Ok((left, right));
}

pub fn part1(path: &str) -> Result<Int, Box<dyn Error>> {
    let (mut left, mut right) = load_data(path)?;

    left.sort();
    right.sort();
    
    let result = left.iter().zip(right.iter()).map(|(a, b)| (a - b).abs()).sum::<Int>();

    return Ok(result);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_part1_works() -> Result<(), Box<dyn Error>> {
        let path = "src/day1/part1_test.dat";
        let result = part1(path)?;
    
        assert_eq!(11, result);

        return Ok(());
    }
}
