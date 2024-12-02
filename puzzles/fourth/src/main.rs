use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};

use regex::Regex;

fn main() {
    println!("\n----- Advent of Code 2024 - Puzzle 4 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/fourth/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let safe_report_count = path
        .read_reports()
        .expect("Failed to read safe reports")
        .count_valid();

    println!("Answer: {safe_report_count}");
    println!("----- ------------------------------ -----");
}

trait ReadReports {
    fn read_reports(&self) -> Result<Vec<Vec<i32>>, Box<dyn Error>>;
}

impl ReadReports for Path {
    fn read_reports(&self) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
        let regex = Regex::new(r"\d+")?;
        let file = fs::File::open(self)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);
        let mut reports = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let matches = regex.find_iter(&line);
            let mut report = Vec::<i32>::new();

            for measurement in matches {
                let measurement = measurement.as_str().parse::<i32>()?;
                report.push(measurement);
            }

            reports.push(report);
        }

        Ok(reports)
    }
}

trait ValidateReportsWithProblemDampener {
    fn count_valid(&self) -> u32;
}

impl ValidateReportsWithProblemDampener for Vec<Vec<i32>> {
    fn count_valid(&self) -> u32 {
        let mut count: u32 = 0;

        for report in self {
            if is_valid_report(report, false) {
                count += 1;
            }
        }

        count
    }
}

fn is_valid_report(report: &[i32], dampening_applied: bool) -> bool {
    let mut is_incrementing: Option<bool> = None;

    for (i, curr) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let prev = report[i - 1];
        let diff = *curr as i32 - prev as i32;

        if !(1..=3).contains(&diff.abs()) {
            if !dampening_applied {
                let mut new_report: Vec<i32> = report.into();
                new_report.remove(i - 1);
                return is_valid_report(&new_report, true);
            }

            return false;
        }

        match prev.cmp(curr) {
            std::cmp::Ordering::Less => {
                if is_incrementing.is_some_and(|v| !v) {
                    if !dampening_applied {
                        let mut new_report: Vec<i32> = report.into();
                        new_report.remove(i - 1);
                        return is_valid_report(&new_report, true);
                    }

                    return false;
                }

                is_incrementing = Some(true);
            },
            std::cmp::Ordering::Greater => {
                if is_incrementing.is_some_and(|v| v) {
                    if !dampening_applied {
                        let mut new_report: Vec<i32> = report.into();
                        new_report.remove(i - 1);
                        return is_valid_report(&new_report, true);
                    }

                    return false;
                }

                is_incrementing = Some(false);
            },
            _ => {}
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_report_with_dampening() {
        // 7 6 4 2 1
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_valid_report(&report, false));

        // 1 2 7 8 9
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_valid_report(&report, false));
        
        // 9 7 6 2 1
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_valid_report(&report, false));
        
        // 1 3 2 4 5
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_valid_report(&report, false));
        
        // 8 6 4 4 1
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_valid_report(&report, false));        
        
        // 1 3 6 7 9
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_valid_report(&report, false));
    }

    #[test]
    fn test_is_valid_report_without_dampening() {
        // 7 6 4 2 1
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_valid_report(&report, true));

        // 1 2 7 8 9
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_valid_report(&report, true));

        // 9 7 6 2 1
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_valid_report(&report, true));

        // 1 3 2 4 5
        let report = vec![1, 3, 2, 4, 5];
        assert!(!is_valid_report(&report, true));

        // 8 6 4 4 1
        let report = vec![8, 6, 4, 4, 1];
        assert!(!is_valid_report(&report, true));

        // 1 3 6 7 9
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_valid_report(&report, true));
    }
}