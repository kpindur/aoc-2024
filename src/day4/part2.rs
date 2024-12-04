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

pub fn part2(path: &str) -> Result<Int, Box<dyn Error>> {
    let grid = load_data(path)?;

    let (rows, cols) = (grid.len(), grid[0].len());
    let neighbors = [
        (-1, -1),   (1, -1),
        (-1, 1),    (1, 1)
    ];

    let mut result = 0;

    for i in 0..rows{
        for j in 0..cols {
            let (mut count_s, mut count_m): (Int, Int) = (2, 2);
            let (mut diag_s_i, mut diag_s_j): (Int, Int) = (0, 0);
            let (mut diag_m_i, mut diag_m_j): (Int, Int) = (0, 0);

            if grid[i][j] != 'A' { continue; }
            for (di, dj) in neighbors {
                let (i, j) = (i as Int, j as Int);
                let (new_i, new_j) = (i + di, j + dj);
                let valid_coord = (new_i >= 0 && new_i < rows as Int) && (new_j >= 0 && new_j < cols as Int);
                if !valid_coord { continue; }

                let (new_i, new_j) = (new_i as usize, new_j as usize);
                match grid[new_i][new_j] {
                    'M' => {
                        count_m -= 1;
                        diag_m_i += di;
                        diag_m_j += dj;
                    }
                    'S' => {
                        count_s -= 1;
                        diag_s_i += di;
                        diag_s_j += dj;
                    }
                    _ => continue,
                }
            }
            let proper_no_sm = count_m == 0 && count_s == 0;
            let not_on_diag = (diag_m_i + diag_m_j != 0) && (diag_s_i + diag_s_j != 0);
            if proper_no_sm && not_on_diag { result += 1; }
        }
    }

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

        assert_eq!(9, result);

        return Ok(());
    }
}
