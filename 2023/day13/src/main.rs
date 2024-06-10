use std::{fs, collections::HashSet};

#[derive(Debug)]
struct ReflectLine {
    position: usize,
    direction: char,
    is_valid: bool
}

#[derive(Debug)]
struct Pattern {
    grid: Vec<String>,
    reflection_lines: HashSet<ReflectLine>

}

impl Pattern {
    fn build(pattern_str: &str) -> Pattern {
        let grid = pattern_str.lines().map(|line| line.to_owned()).collect();
        let reflection_lines = HashSet::new();

        for (idx, line) in self.grid.iter().enumerate() {

        }
        Pattern { grid }
    }

    fn get_value(&self) -> u32 {

        if let Some(value) = self.find_vertical_reflection() {
            return value
        }

        let a = self.find_horizontal_reflection();
        if let None = a {
            dbg!(&self.grid);
        }
            
        a.unwrap_or(0) * 100
    }

    fn find_horizontal_matching_line(&self) -> Option<usize> {
        for (idx, line) in self.grid.iter().enumerate() {
            if line == self.grid.get(idx + 1)? {
                let mut offset = 1;
                loop {
                    if let Some(line_a) = self.grid.get(idx.checked_sub(offset).unwrap_or(usize::MAX)) {
                        if let Some(line_b) = self.grid.get(idx +1 + offset) {
                            if line_a != line_b {
                                break;
                            }
                        } else {
                            return Some(idx)
                        }
                    } else {
                        return Some(idx)
                    }
                    offset += 1;
                }
                // dbg!(&self.grid);
                // dbg!(idx);
            }
        }
        return None
    }

    fn find_horizontal_reflection(&self) -> Option<u32> {
        Some(self.find_horizontal_matching_line()? as u32 + 1)
    }

    fn find_vertical_reflection(&self) -> Option<u32> {

        for idx in 0..self.grid[0].len() {
            if let Some(true) = self.is_vert_equal(idx, idx+1) {
                let mut offset = 1;
                loop {
                    match self.is_vert_equal(idx.checked_sub(offset).unwrap_or(usize::MAX), idx +1 + offset) {
                        Some(false) => break,
                        None => return Some(idx as u32 +1),
                        Some(true) => (),
                    }
                    offset += 1;
                }
            }
        }

        return None

        // dbg!(&self.grid);
        // dbg!(start_idx);

    }

    fn is_vert_equal(&self, idx_1: usize, idx_2: usize) -> Option<bool> {
        for line in self.grid.iter() {
            if line.chars().nth(idx_1)? != line.chars().nth(idx_2)? {
                return Some(false)
            }
        }

        return Some(true)
    }
}

#[derive(Debug)]
struct Puzzle {
    patterns: Vec<Pattern>
}

impl Puzzle {
    fn build(input: &str) -> Puzzle {
        let patterns: Vec<Pattern> = input.split("\n\n").map(|pattern_str| Pattern::build(pattern_str)).collect();
        Puzzle { patterns }
    }

    fn part1(&self) -> u32 {
        self.patterns.iter().map(|pattern| pattern.get_value()).sum()
    }

}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let puzzle = Puzzle::build(&input);
    dbg!(puzzle.patterns.len());

    println!("{}", puzzle.part1());
}
