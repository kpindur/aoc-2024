use std::collections::HashMap;
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

pub fn part2(path: &str) -> Result<Int, Box<dyn Error>> {
    let (left, right) = load_data(path)?;

    let mut map = HashMap::<Int, Int>::new();
    for key in right {
        *map.entry(key).or_insert(0) += 1;   
    }
    let result = left.iter().map(|key| key * *map.get(key).unwrap_or(&0)).sum::<Int>();

    return Ok(result);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_part1_works() -> Result<(), Box<dyn Error>> {
        let path = "src/day1/part1_test.dat";
        let result = part2(path)?;
    
        assert_eq!(31, result);

        return Ok(());
    }
}

