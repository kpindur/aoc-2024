use std::{error::Error, fs::File, io::{BufRead, BufReader}};

type Int = i32;

fn load_data(path: &str) -> Result<Vec<Vec<Int>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parsed_lines  = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.split_whitespace()
            .map(|val| val.parse::<Int>().expect("Failed to parse a value!"))
            .collect::<Vec<Int>>()
            )
        .collect::<Vec<Vec<Int>>>();

    return Ok(parsed_lines);
}

fn valid_seq(seq: &[Int]) -> bool {
    let is_valid = seq.windows(2).all(|chunk| (chunk[0] - chunk[1]).abs() < 4 && (chunk[0] - chunk[1]) != 0 );

    let is_monotonic = seq.windows(2).all(|chunk| chunk[0] >= chunk[1]);
    let is_monotonic = seq.windows(2).all(|chunk| chunk[0] <= chunk[1]) || is_monotonic;

    return is_valid && is_monotonic;
}

pub fn part2(path: &str) -> Result<Int, Box<dyn Error>>  {
    let lines = load_data(path)?;
    let mut results = vec![0; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        if valid_seq(line) {
            results[i] = 1;
            continue;
        }

        for j in 0..line.len() {
            let mut fixed_seq = line.to_vec();
            fixed_seq.remove(j);
            if valid_seq(&fixed_seq) {
                results[i] = 1;
                break;
            }

        }
    }
    let result = results.iter().sum();
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_works() -> Result<(), Box<dyn Error>> {
        let path = "src/day2/part1_test.dat";
        let result = part2(path)?;
        
        assert_eq!(4, result);

        return Ok(());
    }
}
