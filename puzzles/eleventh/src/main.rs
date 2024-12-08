use std::{
    collections::HashSet, env, error::Error, fs::File, io::{self, BufRead, BufReader}, path::Path
};


fn main() -> Result<(), Box<dyn Error>> {
    println!("\n----- Advent of Code 2024 - Puzzle 10 -----");

    let path = env::current_dir()
        .map(|dir| dir.join("./puzzles/eleventh/input.txt"))
        .expect("Failed to get current directory");

    println!("Reading file: {:?}", path);
    
    let mut reader = PatrolPathReader::from_file(path)?;

    reader.patrol_path();

    let distinct_positions = reader.get_distinct_positions();

    println!("Answer: {distinct_positions}");
    println!("----- ------------------------------- -----");

    Ok(())
}

enum GuardDirection {
    Up,
    Right,
    Down,
    Left
}

struct PatrolPathReader {
    obstacles: HashSet<(usize, usize)>,
    traversed_positions: HashSet<(usize, usize)>,
    guard_direction: GuardDirection,
    guard_position: (isize, isize),
    map_size: (usize, usize)
}

impl PatrolPathReader {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_position: Option<(isize, isize)> = None;

        let file = File::open(file_path)?;
        let reader = io::BufReader::with_capacity(32 * 1024, file);

        let mut width = 0;
        let mut height = 0;

        for (y_pos, line) in reader.lines().enumerate() {
            let line = line?;

            for (x_pos, char) in line.chars().enumerate() {
                if let '#' = char {
                    obstacles.insert((x_pos, y_pos));
                    continue;
                }

                if let '^' = char {
                    guard_position = Some((x_pos as isize, y_pos as isize));
                    continue;
                }

                if x_pos > width {
                    width = x_pos
                }
            }

            if y_pos > height {
                height = y_pos
            }
        }
        
        Ok(PatrolPathReader {
            guard_position: guard_position.ok_or("Guard position not found")?,
            guard_direction: GuardDirection::Up,
            obstacles,
            traversed_positions: HashSet::new(),
            map_size: (width, height)
        })
    }

    pub fn new(obstacles: HashSet<(usize, usize)>, guard_position: (isize, isize), map_size: (usize, usize)) -> Self {
        Self {
            guard_position,
            obstacles,
            traversed_positions: HashSet::new(),
            guard_direction: GuardDirection::Up,
            map_size
        }
    }

    fn get_next_guard_position(&self) -> (isize, isize) {
        let (curr_x, curr_y) = self.guard_position;

        match self.guard_direction {
            GuardDirection::Up => (curr_x, curr_y - 1),
            GuardDirection::Right => (curr_x + 1, curr_y),
            GuardDirection::Down => (curr_x, curr_y + 1),
            GuardDirection::Left => (curr_x - 1, curr_y),
        }
    }

    fn get_next_guard_direction(&self) -> GuardDirection {
        match self.guard_direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }

    fn guard_is_on_map(&self) -> bool {
        let (x, y) = self.guard_position;

        (x >= 0 && x < self.map_size.0 as isize) && (y >= 0 && y < self.map_size.1 as isize)
    }

    fn patrol_path(&mut self) {
        while self.guard_is_on_map() {
            let (next_x, next_y) = self.get_next_guard_position();

            if !self.is_obstacle((next_x as usize, next_y as usize)) {
                self.traversed_positions.insert((next_x as usize, next_y as usize));
                self.guard_position = (next_x, next_y);
                continue;
            }

            self.guard_direction = self.get_next_guard_direction();
        }
    }

    fn get_distinct_positions(&self) -> usize {
        self.traversed_positions.len()
    }

    fn is_obstacle(&self, position: (usize, usize)) -> bool {
        self.obstacles.contains(&position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        // ....#.....
        // .........#
        // ..........
        // ..#.......
        // .......#..
        // ..........
        // .#..^.....
        // ........#.
        // #.........
        // ......#...

        let mut obstacles = HashSet::new();

        obstacles.insert((4, 0));
        obstacles.insert((9, 1));
        obstacles.insert((2, 3));
        obstacles.insert((7, 4));
        obstacles.insert((1, 6));
        obstacles.insert((8, 7));
        obstacles.insert((0, 8));
        obstacles.insert((6, 9));

        let guard_position = (4, 6);

        let mut patrol_path_reader = PatrolPathReader::new(obstacles, guard_position, (10, 10));

        patrol_path_reader.patrol_path();

        assert_eq!(patrol_path_reader.get_distinct_positions(), 42)
    }
}
