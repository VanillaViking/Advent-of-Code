use std::{fs, collections::{HashMap, VecDeque, HashSet}, process::exit, io};

#[derive(Debug)]
enum Plot {
    ROCK,
    GARDEN
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Plot>>,
    start: (usize, usize)
}

impl Map {
    fn build(input: &str) -> Map {
        let mut start: (usize, usize) = (0, 0);
        let grid: Vec<Vec<Plot>> = input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, char)| {
                match char {
                    '.' => Plot::GARDEN,
                    '#' => Plot::ROCK,
                    'S' => {
                        start = (x, y);
                        Plot::GARDEN
                    },
                    _ => panic!()
                }
            }).collect()
        }).collect();

        Map { grid, start }
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let map = Map::build(&input);

    let steps = 64;

    println!("{}", part1(&map, steps));
    
}

fn part1(map: &Map, steps: u32) -> usize {
    let mut seen: HashMap<(usize, usize), u32> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((map.start, 0));

    while queue.len() != 0 {
        let (current_point, current_steps) = queue.pop_front().unwrap();
        if seen.contains_key(&current_point) {
            continue;
        }
        seen.insert(current_point, current_steps);

        if current_steps == steps {
            continue;
        }

        if let Some(row) = map.grid.get(current_point.1) {
            if let Some(plot) = row.get(current_point.0) {
                if let Plot::ROCK = plot {
                    continue;
                }
            }
        }

        if let Some(y) = current_point.1.checked_sub(1) {
            queue.push_back(((current_point.0, y), current_steps+1));
        }
        if let Some(x) = current_point.0.checked_sub(1) {
            queue.push_back(((x, current_point.1), current_steps+1));
        }

        queue.push_back(((current_point.0, current_point.1 + 1), current_steps+1));
        queue.push_back(((current_point.0 + 1, current_point.1), current_steps+1));

        
    }
    let temp: Vec<((usize, usize), u32)> = seen.into_iter().filter(|(point, steps)| {
        if let Plot::GARDEN = map.grid[point.1][point.0] {
            return steps % 2 == 0
        }
        return false
    }).collect();

    dbg!(temp).len()

}
