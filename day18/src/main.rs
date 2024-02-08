use std::fs;

#[derive(Debug,Clone)]
struct Range {
    startx: i64,
    endx: i64,
    starty: i64,
    endy: i64,
    direction: char
}

#[derive(Debug, Clone)]
enum WallAction {
    Toggle(i64),
    Ignore(i64, i64),
    IngoreToggle(i64, i64)
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        value >= self.startx && value <= self.endx
    }
}

fn parse_input_2(input: &str) -> Vec<(char, i64)> {
    input.lines().map(|line| {
        let hex_str = &line.split(" ").nth(2).unwrap()[1..];
        let direction_str = &hex_str[hex_str.len()-2..];
        let dig_len = i64::from_str_radix(&hex_str[1..hex_str.len()-2], 16).unwrap();
        (direction_str.chars().nth(0).unwrap(), dig_len)
    }).collect()
}

fn construct_ranges(trenches: Vec<(char, i64)>) -> (Vec<Range>, u64) {
    let mut x = 500;
    let mut y: i64 = 500;
    let mut outline_total: u64 = 0;

    let mut range_list: Vec<Range> = Vec::new();

    for trench in trenches.iter() {
        match trench.0 {
            '3' => {
                range_list.push(Range { startx: x, endx: x, starty: y-trench.1, endy: y, direction: trench.0 });
                y-= trench.1;
            },
            '0' => {
                range_list.push(Range { startx: x, endx: x+trench.1, starty: y, endy: y, direction: trench.0 });
                x+= trench.1;
            },
            '1' => {
                range_list.push(Range { startx: x, endx: x, starty: y, endy: y+trench.1, direction: trench.0 });
                y+= trench.1;
            },
            '2' => {
                range_list.push(Range { startx: x-trench.1, endx: x, starty: y, endy: y, direction: trench.0 });
                x-= trench.1;
            },
            _ => panic!(),
        } 
        outline_total += trench.1 as u64
    }
    (range_list, outline_total)

}

fn part2(input: &str) {
    let (ranges, outline_total) = construct_ranges(parse_input_2(input));
    dbg!(&ranges, outline_total);
    let max_value = ranges.iter().max_by_key(|range| range.endx).unwrap().endx;
    let min = ranges.iter().min_by_key(|range| range.endx).unwrap().endx;
    dbg!(max_value, min);
    let mut wall_columns: Vec<Vec<WallAction>> = Vec::new();

    for col in -5000000..=max_value {
        wall_columns.push(ranges.iter().filter_map(|range| {
            if !range.contains(col) {
                return None;
            }
            if col % 100000 == 0 {
                        dbg!(col);
                    }

            match range.direction {
                '0' | '2' => {
                    if let Some(_) = ranges.iter().filter(|r| (r.direction == '1' || r.direction == '3') && r.startx == col).find(|r| r.starty == range.starty || r.endy == range.starty) {
                        return None
                    } else {
                        Some(WallAction::Toggle(range.starty))
                    }
                },
                '1' | '3' => {
                    // let vert = ranges.iter().filter(|r| (r.direction == '1' || r.direction == '3') && r.startx = col)
                    let horizontals: Vec<Range> = ranges.iter().filter(|r| (r.direction == '0' || r.direction == '2')).map(|r| r.to_owned()).collect();


                    if horizontals.iter().find(|h| range.starty == h.starty && (h.startx == range.startx || h.endx == range.startx)).unwrap().contains(range.startx+1) == horizontals.iter().find(|h| range.endy == h.starty && (h.startx == range.startx || h.endx == range.startx)).unwrap().contains(range.startx +1) {
                        Some(WallAction::Ignore(range.starty, range.endy))
                    } else {
                        Some(WallAction::IngoreToggle(range.starty, range.endy))
                    }
                },
                _ => panic!()
            }
        }).collect());
    }

    let mut total: u64 = 0;
    for col in wall_columns.iter_mut() {
        col.sort_by_key(|wa| {
            match wa {
                WallAction::Toggle(val) => val.to_owned(),
                WallAction::Ignore(val, _) => val.to_owned(),
                WallAction::IngoreToggle(val, _) => val.to_owned(),
            }
        });
        
        total += calculate_col(col);
    }
    // dbg!(outline_total, total);
    dbg!(total + outline_total);
    // dbg!(&wall_columns[725], calculate_col(&mut wall_columns[725].to_vec()));
}

fn calculate_col(col: &mut Vec<WallAction>) -> u64 {
    let mut total: u64 = 0;
    let mut is_countable = false;
    let mut prev_value: i64 = 0;
    for action in col {
        match action {
            WallAction::Toggle(val) => {
                if is_countable {
                    is_countable = false;
                    total += u64::try_from(val.to_owned() - prev_value - 1).unwrap()
                } else {
                    is_countable = true;
                    prev_value = val.to_owned();
                }
            },
            WallAction::Ignore(start, end) => {
                let diff = end.to_owned() - start.to_owned();
                if is_countable {
                    total += u64::try_from(start.to_owned() - prev_value - 1).unwrap();
                    prev_value = end.to_owned();
                }
            },
            WallAction::IngoreToggle(start, end) => {
                if is_countable {
                    total += u64::try_from(start.to_owned() - prev_value - 1).unwrap();
                    is_countable = false;
                } else {
                    prev_value = end.to_owned();
                    is_countable = true;
                }

            }
        }
    }
    total
}

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| {
            print!("{}", c);
        });
        print!("\n");
    })
}

fn trace_plan(grid: &mut Vec<Vec<char>>, input: &str) {
    let directions = parse_input_2(input);

    let mut curr: (usize, usize) = (500, 500);
    grid[curr.1][curr.0] = '#';

    for direction in directions.iter() {

        for _ in 0..direction.1 {
            match direction.0 {
                '3' => curr.0 -= 1,
                '0' => curr.1 += 1,
                '1' => curr.0 += 1,
                '2' => curr.1 -= 1,
                _ => panic!(),
            }

            grid[curr.0][curr.1] = '#';

        }

    }


}

fn parse_input(input: &str) -> Vec<(char, u32)> {
    let directions: Vec<(char, u32)> = input.lines().map(|line| {
        let ch = match line.split(" ").nth(0).unwrap().parse().unwrap() {
            'U' => '3',
            'R' => '0',
            'D' => '1',
            'L' => '2',
            _ => panic!(),
        };
        (ch, line.split(" ").nth(1).unwrap().parse().unwrap())
    }).collect();
    directions
}


fn flood_fill(grid: &mut Vec<Vec<char>>, curr: (usize,usize)) {
    if grid[curr.1][curr.0] != '.' {
        return;
    }
    
    grid[curr.1][curr.0] = '#';
    flood_fill(grid, (curr.0, curr.1 -1));
    flood_fill(grid, (curr.0, curr.1 +1));
    flood_fill(grid, (curr.0 -1, curr.1));
    flood_fill(grid, (curr.0 +1, curr.1));
    

}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    part2(&input);

    // let mut grid = vec![vec!['.'; 1000]; 1000];

    // trace_plan(&mut grid, &input);
    // flood_fill(&mut grid, (450, 200));
    // print_grid(&grid);

}
