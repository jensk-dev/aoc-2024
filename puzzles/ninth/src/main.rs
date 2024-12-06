use std::{
    collections::{HashMap, HashSet}, env, error::Error, fs, io::{self, BufRead}, path::Path
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n----- Advent of Code 2024 - Puzzle 9 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/ninth/input.txt"))
        .expect("Failed to get current directory");


    println!("Reading file: {:?}", path);
    let page_order = PageOrderBuilder::from_file(&path)?.build();
    let sequences = read_sequences(&path)?;

    let middle_number_count = sequences.iter()
        .filter(|sequence| page_order.is_in_order(sequence))
        .map(|sequence| page_order.give_middle_value(sequence))
        .reduce(|a, b| a + b)
        .expect("Failed to reduce");

    println!("Answer: {middle_number_count}");
    println!("----- ------------------------------ -----");

    Ok(())
}

fn read_sequences(file_path: impl AsRef<Path>) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::with_capacity(32 * 1024, file);
    let regex = Regex::new(r"\d+")?;

    let mut sequences = Vec::new();

    for line in reader.lines(){
        let line = line?;

        // while we havent reached an empty line, we keep reading
        if line.is_empty() {
            continue;
        }

        sequences.push(
            regex.captures_iter(&line)
                .map(|capture| capture.get(0).unwrap().as_str().parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?
        )
    }

    Ok(sequences)
}

struct PageOrder {
    rules: HashMap<i32, usize>
}

impl PageOrder {
    pub fn give_in_order(&self, update: &[i32]) -> Vec<i32> {
        let mut update: Vec<i32> = update.into();
        
        update.sort_by(|a, b| {
            let a_index = self.rules.get(a).unwrap_or(&usize::MAX);
            let b_index = self.rules.get(b).unwrap_or(&usize::MAX);

            dbg!(a_index);
            dbg!(b_index);

            a_index.cmp(b_index)
        });

        update
    }

    fn is_in_order(&self, update: &[i32]) -> bool {
        let ordered = self.give_in_order(update);

        dbg!(&ordered);
        dbg!(&update);
    
        ordered == update
    }

    fn give_middle_value(&self, update: &[i32]) -> i32 {
        let middle = update.len() / 2;
        update[middle]
    }
}

struct PageOrderBuilder {
    numbers: HashSet<i32>,
    rules: Vec<(i32, i32)>
}

impl PageOrderBuilder {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);
        let regex = Regex::new(r"^(\d+)\|(\d+)$")?;

        let mut page_order_builder = PageOrderBuilder::new();

        for line in reader.lines(){
            let line = line?;

            if regex.is_match(&line) {
                let captures = regex.captures(&line).unwrap();
                let a = captures.get(1).ok_or("Failed to get first number")?.as_str().parse::<i32>()?;
                let b = captures.get(2).ok_or("Failed to get second number")?.as_str().parse::<i32>()?;
                
                page_order_builder = page_order_builder.add_rule(a, b);
            }
        }

        Ok(page_order_builder)
    }

    pub fn new() -> Self {
        Self {
            numbers: HashSet::new(),
            rules: Vec::new()
        }
    }

    pub fn add_rule(mut self, a: i32, b: i32) -> Self {
        self.rules.push((a, b));
        self.numbers.insert(a);
        self.numbers.insert(b);
        self
    }

    pub fn build(self) -> PageOrder {
        let mut numbers = self.numbers.into_iter().collect::<Vec<i32>>();
        let mut swapped = true;

        while swapped {
            swapped = !self.rules.iter().all(|(a, b)| {
                !Self::swap(&mut numbers, *a, *b)
            });
        }

        PageOrder {
            rules: numbers.into_iter()
                .enumerate()
                .map(|(i, n)| (n, i))
                .collect()
        }
    }

    fn swap(numbers: &mut Vec<i32>, a: i32, b: i32) -> bool {
        let a_index = numbers.iter().position(|&x| x == a);
        let b_index = numbers.iter().position(|&x| x == b);

        if a_index.is_none() || b_index.is_none() {
            return false;
        }

        let a_index = a_index.unwrap();
        let b_index = b_index.unwrap();

        if a_index > b_index {
            numbers.swap(a_index, b_index);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_search_3x3_var1() {
        let page_order = PageOrderBuilder::new()
            .add_rule(47, 53)
            .add_rule(97, 13)
            .add_rule(97, 61)
            .add_rule(97, 47)
            .add_rule(75, 29)
            .add_rule(61, 13)
            .add_rule(75, 53)
            .add_rule(29, 13)
            .add_rule(97, 29)
            .add_rule(53, 29)
            .add_rule(61, 53)
            .add_rule(97, 53)
            .add_rule(61, 29)
            .add_rule(47, 13)
            .add_rule(75, 47)
            .add_rule(97, 75)
            .add_rule(47, 61)
            .add_rule(75, 61)
            .add_rule(47, 29)
            .add_rule(75, 13)
            .add_rule(53, 13)
            .build();
        
        let input = vec![75,47,61,53,29];
        assert!(page_order.is_in_order(&input));
        assert!(page_order.give_middle_value(&input) == 61);

        let input = vec![97,61,53,29,13];
        assert!(page_order.is_in_order(&input));
        assert!(page_order.give_middle_value(&input) == 53);

        let input = vec![75,29,13];
        assert!(page_order.is_in_order(&input));
        assert!(page_order.give_middle_value(&input) == 29);

        let input = vec![75,97,47,61,53];
        assert!(!page_order.is_in_order(&input));

        let input = vec![61,13,29];
        assert!(!page_order.is_in_order(&input));

        let input = vec![97,13,75,29,47];
        assert!(!page_order.is_in_order(&input));
    }
}
