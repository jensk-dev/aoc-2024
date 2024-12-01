use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};

use regex::Regex;

fn main() {
    println!("\n----- Advent of Code 2024 - Puzzle 1 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/first/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let similarity_score: u32 = path
        .read_lists()
        .expect("Failed to read lists")
        .get_similarity_score();

    println!("Answer: {similarity_score}");
    println!("----- ------------------------------ -----");
}

trait ReadLists {
    fn read_lists(&self) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>>;
}

impl ReadLists for Path {
    fn read_lists(&self) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
        let regex = Regex::new(r"^(\d+)\s+(\d+)$")?;
        let mut a = Vec::new();
        let mut b = Vec::new();

        let file = fs::File::open(self)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);

        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = regex.captures(&line) {
                let first_match = captures[1].parse::<i32>()?;
                let second_match = captures[2].parse::<i32>()?;
                a.push(first_match);
                b.push(second_match);
            }
        }

        Ok((a, b))
    }
}

fn sort_asc(numbers: impl IntoIterator<Item = i32>) -> Vec<i32> {
    let mut numbers: Vec<_> = numbers.into_iter().collect();
    numbers.sort_unstable();
    numbers
}

trait SortLists {
    fn sort_lists(self) -> (Vec<i32>, Vec<i32>);
}

impl SortLists for (Vec<i32>, Vec<i32>) {
    fn sort_lists(self) -> (Vec<i32>, Vec<i32>) {
        let (a, b) = self;
        (sort_asc(a), sort_asc(b))
    }
}

trait GetSimilarityScore {
    fn get_similarity_score(self) -> u32;
}


impl GetSimilarityScore for (Vec<i32>, Vec<i32>) {
    fn get_similarity_score(self) -> u32 {
        let (mut left, mut right) = self.sort_lists();
        let mut score = 0;

        let mut left_popped = left.pop().unwrap();
        let mut right_popped = right.pop().unwrap();
        let mut multiplier = 0;

        loop {
            // bugfix: when right has an element that does not exist in 


            let mut new_left_popped = left_popped;
            let mut new_right_popped = right_popped;

            while left_popped == new_right_popped && !right.is_empty() {
                println!("{}, {}", left_popped, new_right_popped);
                multiplier += 1;
                new_right_popped = right.pop().unwrap()
            }

            while left_popped == new_left_popped && !left.is_empty() {
                score += multiplier * left_popped;
                new_left_popped = left.pop().unwrap();
            }

            multiplier = 0;
            left_popped = new_left_popped;
            right_popped = new_right_popped;

            if left.is_empty() {
                break;
            }
        }

        score as u32
    }
}