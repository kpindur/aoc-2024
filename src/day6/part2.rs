use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Int = i32;

pub fn load_data(path: &str) -> Result<Vec<Int>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);


    return Ok(Vec::new());
}

pub fn part2(path: &str) -> Result<Int, Box<dyn Error>> {
    let _ = load_data(path)?;

    let result = 0;

    return Ok(result);
}

#[cfg(test)]
mod test{
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_part2_works() -> Result<(), Box<dyn Error>> {
        let current_file = file!();
        let current_dir = PathBuf::from(current_file);

        let prefix = current_dir.parent().expect("Something went terribly wrong...");

        let result = part2(prefix.join("test.dat").to_str().expect("File not found!"))?;

        assert_eq!(0, result);

        return Ok(());
    }
}

