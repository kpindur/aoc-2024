use std::{error::Error, fs::File, io::{BufRead, BufReader}};

type Int = i32;

fn load_data(path: &str) -> Result<Vec<Vec<Int>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parsed_lines = reader.lines();

    return Ok(Vec::new());
}

pub fn part1(path: &str) -> Result<Int, Box<dyn Error>>  {
    let lines = load_data(path)?;
    let mut results = vec![0; lines.len()];

    let result = results.iter().sum();
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_works() -> Result<(), Box<dyn Error>> {
        let path = "src/day3/part1_test.dat";
        let result = part1(path)?;
        
        assert_eq!(2, result);

        return Ok(());
    }
}
