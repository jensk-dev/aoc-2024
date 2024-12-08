use std::{
    collections::{HashMap, HashSet}, env, error::Error, fs, io::{self, BufRead}, path::Path
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n----- Advent of Code 2024 - Puzzle 10 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/tenth/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);
    let page_order = PageOrderBuilder::from_file(&path)?.build();
    let sequences = read_sequences(&path)?;

    let middle_number_count = sequences.iter()
        .filter(|sequence| !page_order.is_in_order(sequence))
        .map(|sequence| page_order.put_in_order(sequence))
        .map(|sequence| page_order.get_middle_value(&sequence))
        .reduce(|a, b| a + b)
        .expect("Failed to reduce");

    println!("Answer: {middle_number_count}");
    println!("----- ------------------------------- -----");

    Ok(())
}

fn read_sequences(file_path: impl AsRef<Path>) -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::with_capacity(32 * 1024, file);
    let match_regex = Regex::new(r"\d+")?;
    let forbidden_regex = Regex::new(r"\d+\|\d+")?;

    let mut sequences = Vec::new();

    for line in reader.lines(){
        let line = line?;

        if forbidden_regex.is_match(&line) {
            continue;
        }

        if line.is_empty() {
            continue;
        }

        sequences.push(
            match_regex.captures_iter(&line)
                .map(|capture| capture.get(0).unwrap().as_str().parse::<isize>())
                .collect::<Result<Vec<isize>, _>>()?
        )
    }
    
    Ok(sequences)
}

struct PageOrder {
    succeeding_pages: HashMap<isize, HashSet<isize>>,
    preceeding_pages: HashMap<isize, HashSet<isize>>
}

impl PageOrder {
    fn is_in_order(&self, update: &[isize]) -> bool {
        for (x_idx, x) in update.iter().enumerate() {
            for (y_idx, y) in update.iter().enumerate() {
                if x_idx == y_idx {
                    continue;
                }

                if x_idx < y_idx {
                    if let Some(set) = self.succeeding_pages.get(x) {
                        if !set.contains(y) {
                            return false;
                        }
                    }
                }

                if x_idx > y_idx {
                    if let Some(set) = self.preceeding_pages.get(x) {
                        if !set.contains(y) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn get_middle_value(&self, update: &[isize]) -> isize {
        let middle = update.len() / 2;
        update[middle]
    }
    
    fn put_in_order(&self, update: &[isize]) -> Vec<isize> {
        let mut ordered = update.to_vec();
        let mut changed = true;

        while changed {
            changed = false;
            let len = ordered.len();

            for x_idx in 0..len {

                let x = ordered[x_idx];

                for y_idx in 0..len {
                    if x_idx == y_idx {
                        continue;
                    }
                    
                    let y = ordered[y_idx];

                    if x_idx < y_idx {
                        if let Some(succeeding_pages) = self.succeeding_pages.get(&x) {
                            if !succeeding_pages.contains(&y) {
                                if let Some(preceeding_pages) = self.preceeding_pages.get(&x) {
                                    if preceeding_pages.contains(&y) {
                                        changed = true;
                                        ordered.swap(x_idx, y_idx);
                                    }
                                }
                            }
                        }
                    }
    
                    if x_idx > y_idx {
                        if let Some(preceeding_pages) = self.preceeding_pages.get(&x) {
                            if !preceeding_pages.contains(&y)  {
                                if let Some(succeeding_pages) = self.succeeding_pages.get(&x) {
                                    if succeeding_pages.contains(&y) {
                                        changed = true;
                                        ordered.swap(x_idx, y_idx);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ordered
    }
}

struct PageOrderBuilder {
    rules: Vec<(isize, isize)>
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
                let a = captures.get(1).ok_or("Failed to get first number")?.as_str().parse::<isize>()?;
                let b = captures.get(2).ok_or("Failed to get second number")?.as_str().parse::<isize>()?;
                
                page_order_builder = page_order_builder.add_rule(a, b);
            }
        }

        Ok(page_order_builder)
    }

    pub fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }

    pub fn add_rule(mut self, a: isize, b: isize) -> Self {
        self.rules.push((a, b));
        self
    }

    pub fn build(self) -> PageOrder {
        let mut succeeding_pages: HashMap<isize, HashSet<isize>> = HashMap::new();
        let mut preceeding_pages: HashMap<isize, HashSet<isize>> = HashMap::new();

        for (left, right) in self.rules {
            if let Some(set) = succeeding_pages.get_mut(&left) {
                set.insert(right);
            } else {
                let mut new_set = HashSet::new();
                new_set.insert(right);
                succeeding_pages.insert(left, new_set);
            }

            if let Some(set) = preceeding_pages.get_mut(&right) {
                set.insert(left);
            } else {
                let mut new_set = HashSet::new();
                new_set.insert(left);
                preceeding_pages.insert(right, new_set);
            }
        }

        PageOrder {
            succeeding_pages,
            preceeding_pages
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_order() {
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
        assert!(page_order.get_middle_value(&input) == 61);

        let input = vec![97,61,53,29,13];
        assert!(page_order.is_in_order(&input));
        assert!(page_order.get_middle_value(&input) == 53);

        let input = vec![75,29,13];
        assert!(page_order.is_in_order(&input));
        assert!(page_order.get_middle_value(&input) == 29);

        let input = vec![75,97,47,61,53];
        assert!(!page_order.is_in_order(&input));

        let input = vec![61,13,29];
        assert!(!page_order.is_in_order(&input));

        let input = vec![97,13,75,29,47];
        assert!(!page_order.is_in_order(&input));
    }

    #[test]
    fn test_page_order_put_in_order() {
        let page_order = PageOrderBuilder::new()
            .add_rule(13, 14)
            .add_rule(14, 15)
            .build();

        let expected = vec![13, 14, 15];

        assert_eq!(page_order.put_in_order(&[15, 14, 13]), expected);
        assert_eq!(page_order.put_in_order(&[15, 13, 14]), expected);
        assert_eq!(page_order.put_in_order(&[14, 15, 13]), expected);
        assert_eq!(page_order.put_in_order(&[14, 13, 15]), expected);
        assert_eq!(page_order.put_in_order(&[13, 14, 15]), expected);
        assert_eq!(page_order.put_in_order(&[13, 15, 14]), expected);
    }
}
