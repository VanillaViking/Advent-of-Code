use std::{fs, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Platform {
    grid: Vec<String>,
    cube_cols: HashMap<usize, Vec<usize>>,
    cube_rows: HashMap<usize, Vec<usize>>,
    rock_positions: Vec<(usize, usize)>

}

impl Platform {
    fn build(input: &str) -> Platform {
        let mut cube_cols = HashMap::new();
        let mut cube_rows = HashMap::new();
        let mut rock_positions = Vec::new();
        let grid = input.lines().enumerate().map(|(row, line)| {
            line.chars().enumerate().for_each(|(col, cell)| {
                match cell {
                    '#' => {
                        cube_cols.entry(col).or_insert(Vec::new()).push(row);
                        cube_rows.entry(row).or_insert(Vec::new()).push(col);
                    },
                    'O' => {
                        rock_positions.push((col, row));
                    },
                    _ => ()
                };
            });
            line.to_owned()
        }).collect();

        Platform { grid, cube_cols, cube_rows, rock_positions }
    }

    fn tilt_north(&mut self) {

        let mut occupied = HashSet::new();
        
        self.rock_positions.sort_by_key(|rock| rock.1);
        self.rock_positions.iter_mut().for_each(|rock| {
            let new_vec = Vec::new();
            let cubes = self.cube_cols.get(&rock.0).unwrap_or(&new_vec); 
            let mut stopped_at = find_predecessor(cubes, rock.1).unwrap_or(0);
            
            loop {
                if rock.1 == stopped_at {
                    occupied.insert((rock.0, stopped_at));
                    break;
                }

                if occupied.contains(&(rock.0, stopped_at)) {
                    stopped_at += 1;
                    continue;
                }
                *rock = (rock.0, stopped_at);
                occupied.insert((rock.0, stopped_at));
                break;
            }
        });
    }

    fn tilt_east(&mut self) {

        let mut occupied = HashSet::new();

        self.rock_positions.sort_by_key(|rock| rock.0);
        self.rock_positions.iter_mut().for_each(|rock| {
            let new_vec = Vec::new();
            let cubes = self.cube_rows.get(&rock.1).unwrap_or(&new_vec); 
            let mut stopped_at = find_predecessor(cubes, rock.0).unwrap_or(0);
            
            loop {
                if rock.0 == stopped_at {
                    occupied.insert((stopped_at, rock.1));
                    break;
                }

                if occupied.contains(&(stopped_at, rock.1)) {
                    stopped_at += 1;
                    continue;
                }
                *rock = (stopped_at, rock.1);
                occupied.insert((stopped_at, rock.1));
                break;
            }
        });
    }

    fn tilt_south(&mut self) {

        let mut occupied = HashSet::new();
        self.rock_positions.sort_by_key(|rock| rock.1);
        self.rock_positions.reverse();

        self.rock_positions.iter_mut().for_each(|rock| {
            let new_vec = Vec::new();
            let cubes = self.cube_cols.get(&rock.0).unwrap_or(&new_vec); 
            let mut stopped_at = find_successor(cubes, rock.1).unwrap_or(self.grid.len()-1);
            
            loop {
                if rock.1 == stopped_at {
                    occupied.insert((rock.0, stopped_at));
                    break;
                }

                if occupied.contains(&(rock.0, stopped_at)) {
                    stopped_at -= 1;
                    continue;
                }
                *rock = (rock.0, stopped_at);
                occupied.insert((rock.0, stopped_at));
                break;
            }
        });
    }

    fn tilt_west(&mut self) {

        let mut occupied = HashSet::new();

        self.rock_positions.sort_by_key(|rock| rock.0);
        self.rock_positions.reverse();

        self.rock_positions.iter_mut().for_each(|rock| {
            let new_vec = Vec::new();
            let cubes = self.cube_rows.get(&rock.1).unwrap_or(&new_vec); 
            let mut stopped_at = find_successor(cubes, rock.0).unwrap_or(self.grid[0].len() -1);
            
            loop {
                if rock.0 == stopped_at {
                    occupied.insert((stopped_at, rock.1));
                    break;
                }

                if occupied.contains(&(stopped_at, rock.1)) {
                    stopped_at -= 1;
                    continue;
                }
                *rock = (stopped_at, rock.1);
                occupied.insert((stopped_at, rock.1));
                break;
            }
        });
    }

    fn calculate_stress(&self) -> usize {
        self.rock_positions.iter().map(|rock| {
            self.grid.len() - rock.1     
        }).sum()
    }

    fn display(&self) {
         let mut empty_grid: Vec<String> = self.grid.iter().map(|line| line.replace("O", ".")).collect();

        self.rock_positions.iter().for_each(|rock| {
            empty_grid[rock.1].replace_range(rock.0..rock.0+1, "O");
        });

    }
}

fn find_predecessor(arr: &Vec<usize>, target: usize) -> Option<usize> {
    if arr.len() == 0 {
        return None
    }

    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut predecessor: Option<usize> = None;

    while low <= high {
        let mid = (high + low) / 2;

        if arr[mid] < target {
            predecessor = Some(arr[mid] + 1);
            low = mid + 1;
        } else {
            high = mid.checked_sub(1)?;
        }
    }
    predecessor
}

fn find_successor(arr: &Vec<usize>, target: usize) -> Option<usize> {
    if arr.len() == 0 {
        return None
    }

    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut successor: Option<usize> = None;

    while low <= high {
        let mid = (high + low) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else {
            successor = Some(arr[mid] - 1);
            if let Some(n) = mid.checked_sub(1) {
                high = n;
            } else {
                break;
            }
        }
    }

    successor
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut platform = Platform::build(&input);
    for n in 1..1000000000 {
        platform.tilt_north();
        platform.tilt_east();
        platform.tilt_south();
        platform.tilt_west();
        println!("{}: {}", platform.calculate_stress(), n);
            if n % 1000 == 0 {
        }
    }
}
