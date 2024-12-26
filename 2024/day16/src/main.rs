use std::{cmp::{self, Ordering}, collections::{BinaryHeap, HashSet, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("sample2").unwrap();
    
    part1_try2(&input);
    part2(&input);
}

enum Direction {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 3,
}


#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct State {
    points: u32,
    just_turned: bool,
    distance: u32,
    orientation: Orientation,
}


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Orientation {
    position: (usize, usize),
    facing: u32,
}

// Implement `Ord` and `PartialOrd` for custom ordering
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order for a min-heap
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(position: (usize, usize)) -> Self {
        Self { orientation: Orientation { position, facing: 1 }, points: 0, just_turned: false, distance: u32::MAX }
    }
}


fn part1_try2(input: &str) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0,0);
    let mut end = (0,0);

    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                start = (x, y);
            }
            if *ch == 'E' {
                end = (x, y);
            }
        }
    }

    print_map(&map);
    dbg!(start);
    dbg!(end);

    let starting_state = State::new(start);

    let mut heap = BinaryHeap::new();
    heap.push(starting_state);

    let mut seen = HashSet::new();

    let mut min_points = u32::MAX;

    while !heap.is_empty() {
        let curr_state = heap.pop().unwrap();


        if seen.contains(&curr_state.orientation) {
            continue;
        }
        seen.insert(curr_state.orientation.clone());
        if map[curr_state.orientation.position.1][curr_state.orientation.position.0] == '#' {
            continue;
        }

        if map[curr_state.orientation.position.1][curr_state.orientation.position.0] == 'E' {
            dbg!(cmp::min(curr_state.points, min_points));
            min_points = cmp::min(curr_state.points, min_points);
            seen.remove(&curr_state.orientation);
            continue;
        }

        let new_pos = move_facing(curr_state.orientation.position, curr_state.orientation.facing);
        let clock = move_facing(curr_state.orientation.position, (curr_state.orientation.facing +1)%4);
        let anti_clock = move_facing(curr_state.orientation.position, (curr_state.orientation.facing.checked_sub(1).unwrap_or(3)) % 4);
        let uturn =  move_facing(curr_state.orientation.position, (curr_state.orientation.facing + 2) % 4);
        let forward_dist = new_pos.0.abs_diff(end.0) + new_pos.1.abs_diff(end.1);
        let clock_dist = clock.0.abs_diff(end.0) + clock.1.abs_diff(end.0);
        let anti_clock_dist = anti_clock.0.abs_diff(end.0) + anti_clock.1.abs_diff(end.0);
        let uturn_dist = uturn.0.abs_diff(end.0) + uturn.1.abs_diff(end.0);

        //forward
        heap.push(State { orientation: Orientation {position: new_pos, facing: curr_state.orientation.facing }, points: curr_state.points +1, just_turned: false, distance: forward_dist as u32});
        if !curr_state.just_turned {
            //turn 90
            heap.push(State { orientation: Orientation { position: curr_state.orientation.position, facing: (curr_state.orientation.facing +1) % 4 }, points: curr_state.points + 1000, just_turned: true, distance: clock_dist as u32 });
            heap.push(State { orientation: Orientation { position: curr_state.orientation.position, facing: (curr_state.orientation.facing.checked_sub(1).unwrap_or(3)) % 4 }, points: curr_state.points + 1000, just_turned: true, distance: anti_clock_dist as u32 });
            //turn 180
            heap.push(State { orientation: Orientation { position: curr_state.orientation.position, facing: (curr_state.orientation.facing +2) % 4 }, points: curr_state.points + 2000, just_turned: true, distance: uturn_dist as u32 });
        }


    }


}


fn part2(input: &str) {
}

fn move_facing(position: (usize, usize), facing: u32) -> (usize,usize) {
       match facing {
           0 => (position.0, position.1 - 1),
           1 => (position.0 + 1, position.1),
           2 => (position.0, position.1 + 1),
           3 => (position.0 - 1, position.1),
           _ => panic!(),
       }
}

fn print_map(map: &[Vec<char>]) {
    let str_map: Vec<String> = map.iter().map(|v| v.iter().collect::<String>()).collect();
    dbg!(str_map);
}
