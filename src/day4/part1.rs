use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Int = i32;

pub fn load_data(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let parsed_line = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    return Ok(parsed_line);
}

fn dfs(grid: &[Vec<char>], word: &[char], position: (usize, usize), direction: (Int, Int), word_index: usize) -> bool {
    if word_index == word.len() { return true; }
    let (x, y) = (position.0 as Int, position.1 as Int);
    let (dx, dy) = direction;

    let (new_x, new_y) = (x + dx, y + dy);
    let valid_coord = new_x >= 0 && new_x < grid.len() as Int && new_y >= 0 && new_y < grid[0].len() as Int;
    if !valid_coord { return false; }

    let (new_x, new_y) = (new_x as usize, new_y as usize);

    if grid[new_x][new_y] != word[word_index] { return false; }

    return dfs(grid, word, (new_x, new_y), direction, word_index+1);
}

pub fn part1(path: &str) -> Result<Int, Box<dyn Error>> {
    let grid = load_data(path)?;

    let (rows, cols) = (grid.len(), grid[0].len());
    let neighbors = [
        (-1, -1),   (0, -1),    (1, -1),
        (-1, 0),                (1, 0),
        (-1, 1),    (0, 1),     (1, 1)
    ];

    let word = &['X', 'M', 'A', 'S'];
    let mut result = 0;

    for i in 0..rows{
        for j in 0..cols {
            if grid[i][j] != word[0] { continue; }
            for neighbor in neighbors {
                if dfs(&grid, word, (i, j), neighbor, 1) {
                    result += 1;
                }
            }
        }
    }

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

        assert_eq!(18, result);

        return Ok(());
    }
}
