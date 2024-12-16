use std::{collections::HashSet, fs, thread::sleep, time::Duration};

fn main() {
    let input = fs::read_to_string("sample2").unwrap();
    let (map_str, moves) = input.split_once("\n\n").unwrap();

    let mut map: Vec<Vec<char>> = map_str.lines().map(|s| s.chars().collect()).collect();

    let mut part2_map: Vec<Vec<char>> = map.iter().map(|row| {
        let mut new_row = Vec::new();
        row.iter().for_each(|tile| {
            match tile {
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                },
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                },
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                },
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!(),
            }
        });
        new_row
    }).collect();

    let mut bot_coords = (0,0);
    for (y, row) in map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if *n == '@' {
                bot_coords = (x, y);
            }
        }
    }
    
    for direction in moves.trim().chars() {
        move_bot(&mut map, &mut bot_coords, direction);
        // sleep(Duration::from_millis(200));
        // print!("\x1B[2J");
        // print_map(&map);
        // dbg!(direction);
    }
    
    let mut total = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if *n == 'O' {
                total;
                total += dbg!((y * 100) + x);
            }
        }
    }

    dbg!(&total);

    print_map(&part2_map);

    for (y, row) in part2_map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if *n == '@' {
                bot_coords = (x, y);
            }
        }
    }

    for direction in moves.trim().chars() {
        move_bot2(&mut part2_map, &mut bot_coords, direction);
        sleep(Duration::from_millis(200));
        print!("\x1B[2J");
        print_map(&part2_map);
        dbg!(direction);
    }

    let mut total2 = 0;
    for (y, row) in part2_map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if *n == '[' {
                total2 += (y * 100) + x;
            }
        }
    }
    print_map(&part2_map);
    dbg!(total2);

}

fn move_bot2(part2_map: &mut Vec<Vec<char>>, bot_coords: &mut (usize, usize), direction: char) {
    let mut movement = (0 as i32,0 as i32);

    match direction {
        '>' => movement = (1,0),
        '<' => movement = (-1,0),
        '^' => movement = (0,-1),
        'v' => movement = (0,1),
        '\n' => return,
        _ => {
            dbg!(direction);
            panic!()
        },
    }

    let next_coord = ((bot_coords.0 as i32 + movement.0) as usize, (bot_coords.1 as i32 + movement.1) as usize);
    let next_tile = part2_map[next_coord.1][next_coord.0];

    let mut coords = HashSet::new();
    coords.insert(next_coord);

    match next_tile {
        '#' => return,
        '.' => {
            part2_map[bot_coords.1][bot_coords.0] = '.';
            part2_map[next_coord.1][next_coord.0] = '@';
            *bot_coords = next_coord;
        },
        '[' => {
            if move_next(coords, movement, part2_map, direction) {
                part2_map[bot_coords.1][bot_coords.0] = '.';
                part2_map[next_coord.1][next_coord.0] = '@';
                *bot_coords = next_coord;
            }
        },
        ']' => {
            if move_next(coords, movement, part2_map, direction) {
                part2_map[bot_coords.1][bot_coords.0] = '.';
                part2_map[next_coord.1][next_coord.0] = '@';
                *bot_coords = next_coord;
            }
        }
        _ => {
            dbg!(next_tile);
            print_map(&part2_map);
            panic!()
        },
    }
}

fn move_next(coords: HashSet<(usize, usize)>, movement: (i32, i32), part2_map: &mut Vec<Vec<char>>, direction: char) -> bool {
    let mut next_coords = HashSet::new();
    let mut corrected_coords = HashSet::new();

    if coords.is_empty() {
        return true;
    }
    if coords.iter().all(|c| part2_map[c.1][c.0] == '.') {
        return true;
    }

    for n in coords.iter() {
        if part2_map[n.1][n.0] == ']' {
            corrected_coords.insert((n.0 -1, n.1));
        } else {
            corrected_coords.insert(n.clone());

        }
    }

    for c in corrected_coords.iter() {
        if part2_map[c.1][c.0] == '.' {
            continue;
        }
        if part2_map[c.1][c.0] == '#' {
            return false;
        }
        get_next(c, direction).iter().for_each(|coord| {
            next_coords.insert(coord.to_owned());
        });
    }

    if move_next(next_coords, movement, part2_map, direction) {
        for coord in corrected_coords.iter() {
            if part2_map[coord.1][coord.0] == '.' {
                continue;
            }
            match direction {
                '>' => {
                    part2_map[coord.1][coord.0 + 2] = ']';
                    part2_map[coord.1][coord.0 + 1] = '[';
                    part2_map[coord.1][coord.0] = '.';
                },
                '<' => {
                    dbg!(coord);
                    part2_map[coord.1][coord.0 - 1] = '[';
                    part2_map[coord.1][coord.0] = ']';
                    part2_map[coord.1][coord.0+1] = '.';
                },
                '^' => {
                    part2_map[coord.1 -1][coord.0+1] = ']';
                    part2_map[coord.1 -1][coord.0] = '[';
                    part2_map[coord.1][coord.0 +1] = '.';
                    part2_map[coord.1][coord.0] = '.';
                },
                'v' => {
                    part2_map[coord.1 +1][coord.0 +1] = ']';
                    part2_map[coord.1 +1][coord.0] = '[';
                    part2_map[coord.1][coord.0 +1] = '.';
                    part2_map[coord.1][coord.0] = '.';
                },
                _ => panic!(),
            };

        }
        return true
        
    } else {
        return false
    }
}

fn get_next(c: &(usize, usize), direction: char) -> Vec<(usize, usize)> {
    let mut next_coords = Vec::new();
    match direction {
        '>' => next_coords.push((c.0  + 2, c.1)),
        '<' => next_coords.push((c.0 -1, c.1)),
        '^' => {
            next_coords.push((c.0, c.1 -1));
            next_coords.push((c.0+1, c.1 -1));
        },
        'v' => {
            next_coords.push((c.0, c.1 +1));
            next_coords.push((c.0+1, c.1 +1));
        },
        _ => panic!(),
    };

    next_coords
}

fn print_map(map: &[Vec<char>]) {
    let str_map: Vec<String> = map.iter().map(|v| v.iter().collect::<String>()).collect();
    dbg!(str_map);
}

fn move_bot(map: &mut Vec<Vec<char>>, bot_coords: &mut (usize, usize), direction: char) {
    let mut movement = (0 as i32,0 as i32);
    match direction {
        '>' => movement = (1,0),
        '<' => movement = (-1,0),
        '^' => movement = (0,-1),
        'v' => movement = (0,1),
        '\n' => return,
        _ => {
            dbg!(direction);
            panic!()
        },
    }
    
    let next_coord = ((bot_coords.0 as i32 + movement.0) as usize, (bot_coords.1 as i32 + movement.1) as usize);

    let next_tile = map[next_coord.1][next_coord.0];

    match next_tile {
        '#' => return,
        '.' => {
            map[bot_coords.1][bot_coords.0] = '.';
            map[next_coord.1][next_coord.0] = '@';
            *bot_coords = next_coord;
        },
        'O' => {
            if let Some(c) = check_next(next_coord, movement, map) {
                map[c.1][c.0] = 'O';
                map[next_coord.1][next_coord.0] = '@';
                map[bot_coords.1][bot_coords.0] = '.';
                *bot_coords = next_coord;
            } else {
                return
            }
        }
        _ => {
            dbg!(next_tile);
            print_map(map);
            panic!()
        },
    }
}

fn check_next(coord: (usize, usize), movement: (i32, i32), map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let next_coord = ((coord.0 as i32 + movement.0) as usize, (coord.1 as i32 + movement.1) as usize);
    match map[next_coord.1][next_coord.0] {
        '#' => return None,
        '.' => return Some(next_coord),
        'O' => return check_next(next_coord, movement, map),
        _ => panic!()
    }


}
