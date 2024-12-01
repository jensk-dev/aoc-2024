use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};
use regex::Regex;

fn main() {
    println!("\n----- Advent of Code 2024 - Puzzle 1 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/first/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let sum: u32 = path
        .read_lists()
        .expect("Failed to read lists")
        .sort_lists()
        .calc_distances()
        .expect("Failed to calculate distances")
        .iter()
        .sum();

    println!("Answer: {sum}");
    println!("----- ------------------------------ -----");
}

fn calc_distance(a: i32, b: i32) -> u32 {
    a.abs_diff(b)
}

/// Sort numbers in ascending order
fn sort_asc(numbers: impl IntoIterator<Item = i32>) -> Vec<i32> {
    let mut numbers: Vec<_> = numbers.into_iter().collect();
    numbers.sort_unstable();
    numbers
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

trait SortLists {
    fn sort_lists(self) -> (Vec<i32>, Vec<i32>);
}

impl SortLists for (Vec<i32>, Vec<i32>) {
    fn sort_lists(self) -> (Vec<i32>, Vec<i32>) {
        let (a, b) = self;
        (sort_asc(a), sort_asc(b))
    }
}

trait CalcDistances {
    fn calc_distances(&self) -> Result<Vec<u32>, Box<dyn Error>>;
}

impl CalcDistances for (Vec<i32>, Vec<i32>) {
    fn calc_distances(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        let (a, b) = self;
        if a.len() != b.len() {
            return Err(format!("Lists must be of the same length: {} != {}", a.len(), b.len()).into());
        }

        Ok(a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| calc_distance(x, y))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_distance_b_bigger_than_a() {
        assert_eq!(calc_distance(1, 3), 2);
    }

    #[test]
    fn calc_distance_a_bigger_than_b() {
        assert_eq!(calc_distance(3, 1), 2);
    }

    #[test]
    fn calc_distance_a_equal_to_b() {
        assert_eq!(calc_distance(1, 1), 0);
    }

    #[test]
    fn sort_asc_numbers() {
        let numbers = vec![3, 1, 2];
        assert_eq!(sort_asc(numbers), vec![1, 2, 3]);
    }
}
