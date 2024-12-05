use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n----- Advent of Code 2024 - Puzzle 8 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/eighth/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let word_search = CrossSearch::from_file(path)?.count_matches();

    println!("Answer: {word_search}");
    println!("----- ------------------------------ -----");

    Ok(())
}

struct CrossSearch {
    board: Board,
}

impl CrossSearch {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let board = Board::from_file(file_path)?;

        Ok(CrossSearch { board })
    }

    pub fn count_matches(&mut self) -> u32 {
        let mut count = 0;
        let height = self.board.height;
        let width = self.board.width;

        for y in 0..height as isize {
            for x in 0..width as isize {
                if self.try_match_x_mas((x, y)).is_some() {
                    count += 1;
                }
            }
        }

        count
    }

    fn try_match_x_mas(&self, position: (isize, isize)) -> Option<()> {
        let (x, y) = position;
        let pos = self.board.get_position(x, y)?;

        if pos.char != 'A' {
            return None;
        }

        let top_left = self.board.get_position(x - 1, y - 1);
        let top_right = self.board.get_position(x + 1, y - 1);
        let bottom_left = self.board.get_position(x - 1, y + 1);
        let bottom_right = self.board.get_position(x + 1, y + 1);

        if let (Some(top_left), Some(top_right), Some(bottom_left), Some(bottom_right)) =
            (top_left, top_right, bottom_left, bottom_right)
        {
            if (top_left.char == 'M' && bottom_right.char == 'S'
                || top_left.char == 'S' && bottom_right.char == 'M')
                && (top_right.char == 'M' && bottom_left.char == 'S'
                    || top_right.char == 'S' && bottom_left.char == 'M')
            {
                return Some(());
            }
        }

        None
    }

    #[allow(dead_code)]
    fn from_vecs(board: Vec<Vec<char>>) -> Result<Self, Box<dyn Error>> {
        let height = board.len();
        let width = board[0].len();

        Ok(CrossSearch {
            board: Board {
                width,
                height,
                grid: board
                    .iter()
                    .map(|row| row.iter().map(|&c| BoardPosition { char: c }).collect())
                    .collect(),
            },
        })
    }
}

struct Board {
    width: usize,
    height: usize,
    grid: Vec<Vec<BoardPosition>>,
}

impl Board {
    fn get_position(&self, x: isize, y: isize) -> Option<&BoardPosition> {
        self.grid.get(y as usize)?.get(x as usize)
    }
}

#[derive(Clone, Copy)]
struct BoardPosition {
    char: char,
}

impl Board {
    fn from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut width = 0;
        let mut height = 0;
        let mut board = Vec::new();

        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);

        for (idx, line) in reader.lines().enumerate() {
            let line = line?;

            if idx == 0 {
                width = line.len();
            } else if line.len() != width {
                return Err("All lines must have the same width".into());
            }

            board.push(line.chars().map(|c| BoardPosition { char: c }).collect());
            height += 1;
        }

        Ok(Board {
            width,
            height,
            grid: board,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_search_3x3_var1() {
        let board: Vec<Vec<char>> = vec![vec!['M', '.', 'S'], vec!['.', 'A', '.'], vec![
            'M', '.', 'S',
        ]];

        let mut word_search = CrossSearch::from_vecs(board).unwrap();
        let count = word_search.count_matches();

        assert_eq!(count, 1);
    }

    #[test]
    fn cross_search_3x3_var2() {
        let board: Vec<Vec<char>> = vec![vec!['S', '.', 'M'], vec!['.', 'A', '.'], vec![
            'S', '.', 'M',
        ]];

        let mut word_search = CrossSearch::from_vecs(board).unwrap();
        let count = word_search.count_matches();

        assert_eq!(count, 1);
    }

    #[test]
    fn cross_search_3x3_var3() {
        let board: Vec<Vec<char>> = vec![vec!['S', '.', 'S'], vec!['.', 'A', '.'], vec![
            'M', '.', 'M',
        ]];

        let mut word_search = CrossSearch::from_vecs(board).unwrap();
        let count = word_search.count_matches();

        assert_eq!(count, 1);
    }

    #[test]
    fn cross_search_3x3_var4() {
        let board: Vec<Vec<char>> = vec![vec!['M', '.', 'M'], vec!['.', 'A', '.'], vec![
            'S', '.', 'S',
        ]];

        let mut word_search = CrossSearch::from_vecs(board).unwrap();
        let count = word_search.count_matches();

        assert_eq!(count, 1);
    }

    #[test]
    fn cross_search_example() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let mut word_search = CrossSearch::from_vecs(board).unwrap();
        let count = word_search.count_matches();

        assert_eq!(count, 9);
    }
}
