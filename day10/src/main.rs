use std::{fs, collections::VecDeque};

struct Maze {
    grid: Vec<String>,
    animal: (usize, usize),
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let grid: Vec<String> = input.lines().map(|str| str.to_owned()).collect();
    
    let mut animal: (usize, usize) = (0,0);

    grid.iter().enumerate().for_each(|(y, line)| {
        if let Some(x) = line.find("S") {
            animal = (x, y);
        }
    });

    let maze = Maze { grid, animal };

    println!("{}", part1(&maze));

}

fn part1(maze: &Maze) -> usize {
    // finding start and end of loop
    let mut start_end: Vec<(usize, usize)> = Vec::new();
    if let Some(idx) = maze.animal.1.checked_sub(1) {
        if "|F7".contains(maze.grid[idx].chars().nth(maze.animal.0).unwrap()) {
            start_end.push((maze.animal.0, idx));
        }
    }

    if let Some(x) = maze.animal.0.checked_sub(1) {
        if "-FL".contains(maze.grid[maze.animal.1].chars().nth(x).unwrap()) {
            start_end.push((x, maze.animal.1));
        }
    }

    if let Some(line) = maze.grid.get(maze.animal.1 + 1) {
        if "L|J".contains(line.chars().nth(maze.animal.0).unwrap()) {
            start_end.push((maze.animal.0, maze.animal.1 + 1));
        }
    }
    
    if let Some(char) = maze.grid[maze.animal.1].chars().nth(maze.animal.0 + 1) {
        if "7-J".contains(char) {
            start_end.push((maze.animal.0 + 1, maze.animal.1));
        }
    }

    dbg!(&start_end);

    let (start, end) = (start_end[0], start_end[1]);


    let mut path: Vec<(usize, usize)> = Vec::new();
    path.push(maze.animal);

    trace_path(maze, start, &mut path, end);
    path.len()/2
    
}

fn trace_path(maze: &Maze, point: (usize, usize), path: &mut Vec<(usize, usize)>, end: (usize, usize)) {
    let mut queue = VecDeque::new();
    queue.push_back(point);

    while queue.len() != 0 {
        let point = queue.pop_front().unwrap();

        if point.0 == end.0 && point.1 == end.1 {
            path.push(end);
            continue;
        }
        if path.contains(&point) {
            continue;
        }

        match maze.grid.get(point.1) {
            None => continue,
            Some(line) => {
                match line.chars().nth(point.0) {
                    None => continue,
                    Some('.') => continue,
                    Some('-') => {
                        if let Some(x) = point.0.checked_sub(1) {
                            queue.push_back((x, point.1));
                        };
                        queue.push_back((point.0 + 1, point.1));
                    },
                    Some('|') => {
                        if let Some(y) = point.1.checked_sub(1) {
                            queue.push_back((point.0, y));
                        };
                        queue.push_back((point.0, point.1+1));
                    }
                    Some('F') => {
                        queue.push_back((point.0, point.1+1));
                        queue.push_back((point.0 + 1, point.1))
                    },
                    Some('L') => {
                        if let Some(y) = point.1.checked_sub(1) {
                            queue.push_back((point.0, y));
                        }
                        queue.push_back((point.0+1, point.1));
                    },
                    Some('J') => {
                        if let Some(x) = point.0.checked_sub(1) {
                            queue.push_back((x, point.1));
                        }
                        if let Some(y) = point.1.checked_sub(1) {
                            queue.push_back((point.0, y));
                        }
                    },
                    Some('7') => {
                        if let Some(x) = point.0.checked_sub(1) {
                            queue.push_back((x, point.1));
                        }
                        queue.push_back((point.0, point.1+1));
                    },
                    _ => continue
                }
            }
        }

        path.push(point);

    }

}
