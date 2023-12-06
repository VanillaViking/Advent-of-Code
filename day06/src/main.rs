use std::fs;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn main() {
    
    let input = fs::read_to_string("input").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));

}

fn calculate_winning_ways(race: &Race) -> f64 {

    let discriminant = i64::pow(race.time, 2) - (4*-1*-race.distance);

    let point1 = (race.time as f64 + f64::sqrt(discriminant as f64))/-2.0;
    let point2 = (race.time as f64 - f64::sqrt(discriminant as f64))/-2.0;

    let result = point2.ceil() - point1.floor() -1.0;

    result
}

fn part1(input: &str) -> f64 {

    let input_str: Vec<&str> = input.lines().collect();

    let (_, time_str) = input_str[0].split_once(":").unwrap();
    let (_, distance_str) = input_str[1].split_once(":").unwrap();

    let times: Vec<i32> = time_str.split(" ").filter_map(|num_str| {
        num_str.parse::<i32>().ok()
    }).collect();

    let distances: Vec<i32> = distance_str.split(" ").filter_map(|num_str| {
        num_str.parse::<i32>().ok()
    }).collect();

    let races: Vec<Race> = distances.iter().zip(times).map(|(distance, time)| {
        Race { time: i64::from(time), distance: i64::from(distance.to_owned()) }
    }).collect();

    races.iter().map(|race| {
        calculate_winning_ways(race)
    }).product()
}


fn part2(input: &str) -> f64 {
    let input_str: Vec<&str> = input.lines().collect();

    let (_, time_str) = input_str[0].split_once(":").unwrap();
    let (_, distance_str) = input_str[1].split_once(":").unwrap();

    let time = time_str.replace(" ", "").parse::<i64>().unwrap();
    let distance = distance_str.replace(" ", "").parse::<i64>().unwrap();

    let race = Race { time, distance };

    calculate_winning_ways(&race)

}
