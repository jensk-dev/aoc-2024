use std::{env, error::Error, fs, io::{self, BufRead}, path::Path};

use regex::Regex;

fn main() {
    println!("\n----- Advent of Code 2024 - Puzzle 3 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/third/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);

    let safe_report_count = path.read_safe_reports()
        .expect("Failed to read safe reports");

    println!("Answer: {safe_report_count}");
    println!("----- ------------------------------ -----");
}


trait ReadReports {
    fn read_safe_reports(&self) -> Result<u32, Box<dyn Error>>;
}

impl ReadReports for Path {
    fn read_safe_reports(&self) -> Result<u32, Box<dyn Error>> {
        let regex = Regex::new(r"\d+")?;
        let file = fs::File::open(self)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);

        let mut safe_report_count = 0;
        let mut is_safe: bool = true;

        for line in reader.lines() {
            let line = line?;

            let report = regex.find_iter(&line);
            let mut last_measurement: Option<u32> = None;
            let mut is_incrementing: Option<bool> = None;

            for measurement in report {
                let measurement = measurement.as_str().parse::<u32>()?;

                if let Some(last) = last_measurement {
                    let diff = measurement as i32 - last as i32;

                    if !(1..=3).contains(&diff.abs()) {
                        is_safe = false;
                        break;
                    }


                    match last.cmp(&measurement) {
                        std::cmp::Ordering::Less => {
                            if let Some(false) = is_incrementing {
                                is_safe = false;
                                break;
                            }

                            is_incrementing = Some(true);
                        },
                        std::cmp::Ordering::Greater => {
                            if let Some(true) = is_incrementing {
                                is_safe = false;
                                break;
                            }

                            is_incrementing = Some(false);
                        },
                        _ => {}
                    }

                    
                }

                last_measurement = Some(measurement);
            }

            if is_safe {
                safe_report_count += 1;
            } else {
                is_safe = true;
            }
        }

        Ok(safe_report_count)
    }
}