use std::{fs, cmp};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    input.lines().filter_map(|line| {
        let temp: Vec<&str> = line.split(":").collect();
        let id: u32 = temp[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let mut is_valid = true;
        
        temp[1].split(&[';', ','][..]).fold(Some(id), |_result, group| {
                match group[1..].split_once(" ").unwrap() {
                    (n, "red") => (n.parse::<u32>().unwrap() > red).then(|| is_valid = false),
                    (n, "green") => (n.parse::<u32>().unwrap() > green).then(|| is_valid = false),
                    (n, "blue") => (n.parse::<u32>().unwrap() > blue).then(|| is_valid = false),
                    _ => None,
                };

                is_valid.then_some(id)
            })
    }).sum()
}

fn part2(input: &str) -> u32 {
    
    input.lines().map(|line| {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let (_id_str, game_str) = line.split_once(":").unwrap();

        game_str.split(&[';', ','][..]).for_each(|group| {
            match group[1..].split_once(" ").unwrap() {
                (n, "red") => red = cmp::max(red, n.parse().unwrap()),
                (n, "green") => green = cmp::max(green, n.parse().unwrap()),
                (n, "blue") => blue = cmp::max(blue, n.parse().unwrap()),
                _ => (),
            }
        });

        red * green * blue
    }).sum()

}
