use std::fs;
use itertools::Itertools;



fn main() {
    let input = fs::read_to_string("input").unwrap();

    let readings: Vec<Vec<i64>> = input.lines().map(|line| line.split(" ").map(|num_str| num_str.parse::<i64>().unwrap()).collect()).collect();

    let part1: i64 = readings.iter().map(|reading| extrapolate(&reading)).sum();
    println!("{}", part1);

    let part2: i64 = readings.iter().map(|reading| extrapolate_back(&reading)).sum();
    println!("{}", part2);
}

fn extrapolate(reading: &[i64]) -> i64 {
    if reading.iter().all_equal() {
        return reading[0].to_owned()
    }
    
    let mut derivative: Vec<i64> = Vec::new();
    for (idx, n) in reading.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        derivative.push(n - reading[idx - 1]);
    }

    return reading[reading.len() - 1] + extrapolate(&derivative)
}

fn extrapolate_back(reading: &[i64]) -> i64 {
    if reading.iter().all_equal() {
        return reading[0].to_owned()
    }
    
    let mut derivative: Vec<i64> = Vec::new();
    for idx in (0..reading.len()-1).rev() {
        derivative.push(reading[idx + 1] - reading[idx]);
    }
    derivative.reverse();
    return reading[0] - extrapolate_back(&derivative)
}
