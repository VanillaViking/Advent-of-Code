use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn check_row_is_void(line: &str) -> bool {
    !(line.contains("#"))
}

fn check_col_is_void(grid: &Vec<String>, idx: usize) -> bool {
    grid.iter().find(|line| line.chars().nth(idx).unwrap() == '#').is_none()
}

fn get_distance(a: &Point, b: &Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let grid: Vec<String> = input.lines().map(|str| str.to_owned()).collect();
    let mut points: Vec<Point> = Vec::new();
    let mut y_increment = 0;

    for (y, line) in input.lines().enumerate() {
        if check_row_is_void(line) {
            y_increment += 999999;
            continue;
        }
        let mut x_increment = 0;
        for (x, char) in line.chars().enumerate() {
            if check_col_is_void(&grid, x) {
                x_increment += 999999;
            }
            if char == '#' {
                points.push(Point {x: x + x_increment, y: y + y_increment})
            }
        }
    }

    let mut total = 0;
    let mut count = 0;

    for (idx, point) in points.iter().enumerate() {
        for cmp in idx+1..points.len() {
            total += get_distance(&point, &points[cmp]);
            count += 1;
        }
    }
    println!("{total}");
}
