use std::collections::HashMap;
use std::error::Error;

type Int = i32;

type RuleType = Vec<(Int, Int)>;
type LineType = Vec<Vec<Int>>;

pub fn load_data(path: &str) -> Result<(RuleType, LineType), Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;

    let (rules, ordering) = content.split_once("\n\n").unwrap();
    let rules = rules.lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse::<Int>().expect("Failed to parse first value!"), b.parse::<Int>().expect("Failed to parse second value!"))
        })
        .collect::<Vec<(Int, Int)>>();
    let ordering = ordering.lines()
        .map(|line| line.split(',')
            .map(|v| v.parse::<Int>().expect("Failed to parse values!"))
            .collect::<Vec<Int>>()
        )
        .collect::<Vec<Vec<Int>>>();

    return Ok((rules, ordering));
}

pub fn part1(path: &str) -> Result<Int, Box<dyn Error>> {
    let (rules, lines) = load_data(path)?;

    let mut results = Vec::new();
    
    for line in lines {
        let pages: HashMap<Int, usize> = line.iter().enumerate()
            .map(|(i, &page)| (page, i))
            .collect();

        if rules.iter().all(|(first, second)| {
            match (pages.get(first), pages.get(second)) {
                (Some(&pos1), Some(&pos2)) => pos1 < pos2,
                _ => true
            }
        }) {
            results.push(line);
        }
    }

    let result = results.iter().map(|result| result[&result.len() / 2]).sum();

    return Ok(result);
}

#[cfg(test)]
mod test{
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_part1_works() -> Result<(), Box<dyn Error>> {
        let current_file = file!();
        let current_dir = PathBuf::from(current_file);

        let prefix = current_dir.parent().expect("Something went terribly wrong...");

        let result = part1(prefix.join("test.dat").to_str().expect("File not found!"))?;

        assert_eq!(143, result);

        return Ok(());
    }
}
