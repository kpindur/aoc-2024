use std::{error::Error, fs::File, io::{BufRead, BufReader}};

type Int = i32;

fn load_data(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parsed_lines = reader.lines()
        .map(|line| line.unwrap()).collect();

    return Ok(parsed_lines);
}

enum ParseState {
    LookingForSeq,
    InSequence
}

pub fn part1(path: &str) -> Result<Int, Box<dyn Error>>  {
    let lines = load_data(path)?;
    let mut results = vec![0; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let line: Vec<char> = line.chars().collect();
        let mut j = 0;

        let mut state = ParseState::LookingForSeq;
        let valid_sequence = &['m', 'u', 'l', '('];
        let mut current_sequence = String::new();

        let mut prev_sequence = 0;

        while j < line.len() {
            match state {
                ParseState::LookingForSeq => {
                    if line[j..].starts_with(valid_sequence) {
                        state = ParseState::InSequence;

                        current_sequence.clear();
                        prev_sequence = 0;

                        j += 4;
                        continue;
                    }
                }
                ParseState::InSequence => {
                    match line[j] {
                        ')' => {
                            state = ParseState::LookingForSeq;
                            if current_sequence.is_empty() {
                                j += 1;
                                continue;
                            }
                            if let Ok(num) = current_sequence.parse::<Int>()  {
                                results[i] += num * prev_sequence;
                            } 
                        },
                        ',' => {
                            if current_sequence.is_empty() { state = ParseState::LookingForSeq; }
                            if let Ok(num) = current_sequence.parse::<Int>() {
                                prev_sequence = num;
                            }
                            current_sequence.clear();
                        },
                        '0'..='9' => current_sequence.push(line[j]),
                        _ => state = ParseState::LookingForSeq,
                    }
                }
            }
            j += 1;
        }
    }

    let result = results.iter().sum();
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_part1_works() -> Result<(), Box<dyn Error>> {
        let current_file = file!();
        let current_dir = PathBuf::from(current_file);

        let prefix = current_dir.parent().expect("Something went terribly wrong...");

        let result = part1(prefix.join("test.dat").to_str().expect("File not found!"))?;

        assert_eq!(161, result);

        return Ok(());
    }
}
