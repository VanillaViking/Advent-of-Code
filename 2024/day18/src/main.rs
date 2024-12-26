use std::{collections::{HashSet, VecDeque}, fs};


fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(part1(&input));
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    
    let max = 1024;

    'outer: for (idx, p) in input.lines().enumerate() {
        let (x, y) = p.split_once(",").unwrap();
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';

        if idx <= max {
            continue;
        }

        //bfs

        let mut heap = BinaryHeap::new();
        queue.push_back((0,0,0));

        let mut seen = HashSet::new();

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();

            if seen.contains(&curr) {
                continue;
            }
            seen.insert(curr.clone());

            if curr.0 > 70 || curr.1 > 70 {
                continue;
            }

            if grid[curr.1][curr.0] == '#' {
                continue;
            }


            if curr.0 == 70 && curr.1 == 70 {
                continue 'outer;
            }

            queue.push_back((curr.0 + 1, curr.1, curr.2 + 1));
            queue.push_back((curr.0, curr.1 + 1, curr.2 + 1));
            queue.push_back((curr.0.checked_sub(1).unwrap_or(800), curr.1, curr.2 + 1));
            queue.push_back((curr.0, curr.1.checked_sub(1).unwrap_or(800), curr.2 + 1));
        }

        dbg!(p);

    }
    0
}

fn print_map(map: &[Vec<char>]) {
    let str_map: Vec<String> = map.iter().map(|v| v.iter().collect::<String>()).collect();
    dbg!(str_map);
}
