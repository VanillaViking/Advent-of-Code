use std::{fs, collections::{HashMap, VecDeque, HashSet}, process::exit, io, cmp};

#[derive(Debug)]
enum Plot {
    PATH,
    FOREST,
    SLOPE(char)
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Plot>>,
    start: (usize, usize),
    end: (usize, usize)
}

impl Map {
    fn build(input: &str) -> Map {
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        let grid: Vec<Vec<Plot>> = input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, char)| {
                match char {
                    '.' => Plot::PATH,
                    '#' => Plot::FOREST,
                    '>' | '<' | 'v' | '^' => Plot::SLOPE(char),
                    _ => panic!()
                }
            }).collect()
        }).collect();


        start.0 = grid[0].iter().position(|plot| {if let Plot::PATH = plot {
            return true
        }
        return false}).unwrap();

        end.1 = grid.len() -1;
        end.0 = grid[end.1].iter().position(|plot| {if let Plot::PATH = plot {
            return true
        }
        return false}).unwrap();

        Map { grid, start, end }
    }
}

fn part1(map: &Map) -> u32 {
    let mut cur_path: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut distances_insertion_order: Vec<(usize, usize)> = Vec::new();
    let mut touchpoints: (usize, usize) = (0,0);

    let a = walk(map.start, map, &mut cur_path, &mut distances, &mut distances_insertion_order, &mut touchpoints, 0).unwrap();
    a
    
    //paths.iter().min().unwrap().to_owned()
}

fn walk(current_point: (usize, usize), map: &Map, cur_path: &mut HashSet<(usize, usize)>, distances: &mut HashMap<(usize, usize), u32>, distances_insertion_order: &mut Vec<(usize, usize)>, touchpoints: &mut (usize, usize), current_steps: u32) -> Option<u32> {
    if cur_path.contains(&current_point) {
        return None
    } 
    cur_path.insert(current_point);

    if let Some(dist) = distances.get(&current_point) {
        touchpoints.0 = distances_insertion_order.iter().position(|p| current_point.0 == p.0 && current_point.1 == p.1).unwrap() +1;
        touchpoints.1 = distances_insertion_order.len();
        return Some(dist.to_owned())
    }

    if current_point.0 == map.end.0 && current_point.1 == map.end.1 {
        distances.insert(current_point, 0);
        distances_insertion_order.push(current_point);
        return Some(0);
    }

    if let Some(row) = map.grid.get(current_point.1) {
        if let Some(plot) = row.get(current_point.0) {
            if let Plot::FOREST = plot {
                return None;
            }
            // if let Plot::SLOPE(slope) = plot {
            //     match slope {
            //         '>' => {
            //             let s = walk((current_point.0 +1, current_point.1), map, seen, paths, current_steps +1);
            //             if let Some(val) = s {
            //                 seen.insert(current_point, Some(val+1));
            //                 return Some(val+1);
            //             }
            //             return s;
            //         },
            //         '<' => {
            //             if let Some(x) = current_point.0.checked_sub(1) {
            //                 let s = walk((x, current_point.1), map, seen, paths, current_steps +1);
            //                 if let Some(val) = s {
            //                     seen.insert(current_point, Some(val));
            //                     return Some(val+1)
            //                 }
            //                 return s;
            //             }
            //             return None;
            //         },
            //         'v' => {
            //             let s = walk((current_point.0, current_point.1 + 1), map, seen, paths, current_steps +1);
            //             if let Some(val) = s {
            //                 seen.insert(current_point, Some(val+1));
            //                 return Some(val+1);
            //             }
            //             return s;
            //         },
            //         '^' => {
            //             if let Some(y) = current_point.1.checked_sub(1) {
            //                 let s = walk((current_point.0, y), map, seen, paths, current_steps +1);
            //                 if let Some(val) = s {
            //                     seen.insert(current_point, Some(val+1));
            //                     return Some(val+1);
            //                 }
            //                 return s;
            //             }
            //             return None;
            //         },
            //         _ => panic!(),
            //     }
            // }
        } else {
            return None;
        }
    } else {
        return None;
    }

    let mut max_steps: i32 = -1;

    if let Some(y) = current_point.1.checked_sub(1) {
        if let Some(steps) = walk((current_point.0, y), map, cur_path, distances, distances_insertion_order, touchpoints, current_steps +1) {
            if steps as i32 > max_steps {
                max_steps = steps as i32;
                prune_old_path(distances, distances_insertion_order, touchpoints)
            } else {
                prune_new_path(distances, distances_insertion_order, touchpoints);
            }
        }
    }
    if let Some(x) = current_point.0.checked_sub(1) {
        if let Some(steps) = walk((x, current_point.1), map, cur_path, distances, distances_insertion_order, touchpoints, current_steps +1) {

            if steps as i32 > max_steps {
                max_steps = steps as i32;
                // prune_old_path(distances, distances_insertion_order, touchpoints)

            } else {
                prune_new_path(distances, distances_insertion_order, touchpoints);
            }
        }
    }

    if let Some(steps) = walk((current_point.0, current_point.1 + 1), map, cur_path, distances, distances_insertion_order, touchpoints, current_steps +1) {

        if steps as i32 > max_steps {
            max_steps = steps as i32;
            prune_old_path(distances, distances_insertion_order, touchpoints);
        } else {
            prune_new_path(distances, distances_insertion_order, touchpoints);
        }
    }
    if let Some(steps) = walk((current_point.0 + 1, current_point.1), map, cur_path, distances, distances_insertion_order, touchpoints, current_steps +1) {
        if steps as i32 > max_steps {
            max_steps = steps as i32;
            prune_old_path(distances, distances_insertion_order, touchpoints)
    
        } else {
            prune_new_path(distances, distances_insertion_order, touchpoints);
        }
    }

    cur_path.remove(&current_point);

    if max_steps > -1 {
        distances.insert(current_point, max_steps as u32 + 1);
        distances_insertion_order.push(current_point);
        return Some(max_steps as u32 + 1)
    } else {
        return None;
    }

}

fn prune_old_path(distances: &mut HashMap<(usize, usize), u32>, distances_insertion_order: &mut Vec<(usize, usize)>, touchpoints: &mut (usize, usize)) {
    for point in (touchpoints.0..touchpoints.1).rev() {
        distances.remove(&distances_insertion_order[point]);
        distances_insertion_order.remove(point);
    }
    touchpoints.1 = 0;
    touchpoints.0 = 0;
}

fn prune_new_path(distances: &mut HashMap<(usize, usize), u32>, distances_insertion_order: &mut Vec<(usize, usize)>, touchpoints: &mut (usize, usize)) {
    dbg!(&touchpoints);
    dbg!(&distances_insertion_order.len());
    for point in (touchpoints.1+1..distances_insertion_order.len()).rev() {
        distances.remove(&distances_insertion_order[point]);
        distances_insertion_order.remove(point);
    }
    touchpoints.1 = 0;
    touchpoints.0 = 0;
}


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let map = Map::build(&input);

    println!("{}", part1(&map));

}

