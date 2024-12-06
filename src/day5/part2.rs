use std::{collections::HashMap, error::Error};
use std::cmp::Ordering;

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

struct PageOrder {
    positions: HashMap<Int, usize>
}

impl PageOrder {
    fn new(update: &[Int]) -> Self {
        let positions = update.iter()
            .enumerate()
            .map(|(i, &page)| (page, i))
            .collect();
        return Self { positions };
    }

    fn compare(&self, a: Int, b: Int) -> Option<Ordering> {
        match (self.positions.get(&a), self.positions.get(&b)) {
            (Some(&pos_a), Some(&pos_b)) => Some(pos_a.cmp(&pos_b)),
            _ => None
        }
    }
}

fn is_valid_order(rules: &[(Int, Int)], a: Int, b: Int) -> Option<Ordering> {
    if rules.iter().any(|&(first, second)| first == a && second == b) {
        return Some(Ordering::Less);
    } else if rules.iter().any(|&(first, second)| second == b && first == a) {
        return Some(Ordering::Greater);
    } else { return None; }
}

pub fn part2(path: &str) -> Result<Int, Box<dyn Error>> {
    let (rules, lines) = load_data(path)?;
    
    let mut results = Vec::new();

    for line in &lines {
        let order = PageOrder::new(line);
        let valid_line = rules.iter().all(|&(first, second)| {
            match order.compare(first, second) {
                Some(Ordering::Less) => true,
                Some(_) => false,
                None => true,
            }
        });

        if valid_line { continue; }
        let mut sorted_line = line.clone();
        sorted_line.sort_by(|&a, &b| {
            is_valid_order(&rules, a, b).unwrap_or(Ordering::Equal)
        });

        results.push(sorted_line);
    }

    let result = results.iter().map(|result| result[&result.len() / 2]).sum();

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

        assert_eq!(123, result);

        return Ok(());
    }
}

