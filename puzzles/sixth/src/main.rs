use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};

use regex::Regex;

fn main() {
    println!("\n----- Advent of Code 2024 - Puzzle 6 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/sixth/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let sum = path
        .read_multiplications()
        .expect("Failed to read safe reports")
        .iter()
        .map(|(a, b)| a * b)
        .reduce(|acc, curr| acc + curr)
        .expect("Failed to reduce");

    println!("Answer: {sum}");
    println!("----- ------------------------------ -----");
}

trait ReadMultiplications {
    fn read_multiplications(&self) -> Result<Vec<(i32, i32)>, Box<dyn Error>>;
}

impl ReadMultiplications for Path {
    fn read_multiplications(&self) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
        let regex = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))")?;
        let file = fs::File::open(self)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);
        
        let mut multiplications = Vec::new();
        let mut enabled: bool = true;

        for line in reader.lines() {
            let line = line?;
            let matched = regex.captures_iter(&line);
            
            for mul in matched {
                if mul.get(0).unwrap().as_str() == "don't()" {
                    enabled = false;
                    continue;
                }
                
                if mul.get(0).unwrap().as_str() == "do()" {
                    enabled = true;
                    continue;
                }

                if !enabled {
                    continue;
                }

                let first = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let second = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
                multiplications.push((first, second));
            }
        }

        Ok(multiplications)
    }
}
