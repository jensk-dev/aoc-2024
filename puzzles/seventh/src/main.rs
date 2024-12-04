use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n----- Advent of Code 2024 - Puzzle 7 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/seventh/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let word_search = WordSearchBuilder::from_file(path)?
        .with_word("XMAS")
        .build()?
        .count_matches();
    
    println!("Answer: {word_search}");
    println!("----- ------------------------------ -----");

    Ok(())
}

struct WordSearch<'a> {
    board: Board,
    word_to_match: &'a str,
}

impl WordSearch<'_> {
    pub fn count_matches(&mut self) -> u32 {
        let mut count = 0;
        let height = self.board.height;
        let width = self.board.width;


        for y in 0..height as isize {
            for x in 0..width as isize {
                for velocity in [(1, 0), (0, 1), (1, 1), (1, -1), (-1, 0), (0, -1), (-1, -1), (-1, 1)] {
                    let word: Vec<char> = self.word_to_match.chars().collect();
                    
                    if let Some(matched) = self.try_match_word(word, (x, y), velocity) {
                        if self.board.is_matched(&matched) {
                            continue;
                        }

                       self.board.set_matched(&matched);
                       count += 1;
                    }
                }
            }
        }

        count
    }

    fn try_match_word(&self, word: Vec<char>, position: (isize, isize), velocity: (isize, isize)) -> Option<Vec<BoardPosition>> {
        // iteratively check if the word matches the board in the given direction
        // if it does, return true
        // if it doesn't, return false
        // if the word is exhausted, return true
        // if the board is exhausted, return false

        let mut word: Vec<char> = word;
        let mut position = position;
        let mut positions = Vec::with_capacity(word.len());

        while let Some(char) = word.pop() {
            let board_position = self.board.get_position(position.0, position.1);

            if let Some(board_position) = board_position {
                if !board_position.matches(char) {
                    return None;
                }

                positions.push(*board_position);                
                position = (position.0 + velocity.0, position.1 + velocity.1);
            } else {
                return None;
            }
        }
        
        Some(positions)
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

    fn get_position_mut(&mut self, x: usize, y: usize) -> Option<&mut BoardPosition> {
        self.grid.get_mut(y)?.get_mut(x)
    }

    fn is_matched(&self, positions: &[BoardPosition]) -> bool {
        for position in positions {
            if !position.matches {
                return false;
            }
        }
        true
    }

    fn set_matched(&mut self, positions: &[BoardPosition]) {
        for position in positions {
            if let Some(board_position) = self.get_position_mut(position.x, position.y) {
                board_position.matches = true;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BoardPosition {
    char: char,
    matches: bool,
    x: usize,
    y: usize,
}

impl BoardPosition {
    fn matches(&self, next_char: char) -> bool {
        self.char == next_char
    }
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

            board.push(line.chars().enumerate().map(|(x, c)| BoardPosition { char: c, matches: false, x, y: idx }).collect());
            height += 1;
        }

        Ok(Board {
            width,
            height,
            grid: board
        })
    }
}

struct WordSearchBuilder<'a> {
    board: Option<Board>,
    word_to_match: Option<&'a str>,
}

impl<'a> WordSearchBuilder<'a> {
    fn new() -> Self {
        WordSearchBuilder {
            board: None,
            word_to_match: None
        }
    }

    fn from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let board = Board::from_file(file_path)?;

        Ok(WordSearchBuilder {
            board: Some(board),
            word_to_match: None
        })
    }

    fn from_vecs(board: Vec<Vec<char>>) -> Result<Self, Box<dyn Error>> {
        let width = board.len();
        let height = board[0].len();

        let grid = board
            .into_iter()
            .enumerate()
            .map(|(y, row)| row
                .into_iter()
                .enumerate()
                .map(|(x, c)| BoardPosition { 
                    char: c,
                    matches: false,
                    x,
                    y
                }).collect())
            .collect();

        let board = Board {
            grid,
            width,
            height
        };

        Ok(WordSearchBuilder {
            board: Some(board),
            word_to_match: None
        })
    }

    fn with_word(mut self, word: &'a str) -> Self {
        self.word_to_match = Some(word);
        self
    }

    fn build(self) -> Result<WordSearch<'a>, Box<dyn Error>> {
        let board = self.board.ok_or("Board not set")?;
        let word_to_match = self.word_to_match.ok_or("Word to match not set")?;

        Ok(WordSearch {
            board,
            word_to_match
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search() {
        let board: Vec<Vec<char>> = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
        ];
        
        let word_search = WordSearchBuilder::from_vecs(board)
            .unwrap()
            .with_word("XMAS");

        let count = word_search.build().unwrap().count_matches();

        assert_eq!(count, 18);
    }
}
