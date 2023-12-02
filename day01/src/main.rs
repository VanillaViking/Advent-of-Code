use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
        



    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {

    let mut total = 0;
    input.lines().for_each(|line| {
        let mut number_digits: Vec<u32> = Vec::new();
        line.chars().for_each(|char| {
            if let Some(n) = char.to_digit(10) {
                number_digits.push(n) 
            }
        });
        total += (number_digits.get(0).unwrap_or(&0) * 10) + (number_digits.get(number_digits.len() -1).unwrap_or(&0));
    });

    total
}

fn part2(input: &str) -> u32 {

    let mut total = 0;
    input.lines().for_each(|line| {
        let mut number_digits: Vec<u32> = Vec::new();
        
        let mut new_line = line.replace("one", "o1e");
        new_line = new_line.replace("two", "t2o");
        new_line = new_line.replace("three", "t3e");
        new_line = new_line.replace("four", "f4r");
        new_line = new_line.replace("five", "f5v");
        new_line = new_line.replace("six", "s6x");
        new_line = new_line.replace("seven", "s7n");
        new_line = new_line.replace("eight", "e8t");
        new_line = new_line.replace("nine", "n9e");

        dbg!(&new_line);

        new_line.chars().for_each(|char| {
            if let Some(n) = char.to_digit(10) {
                number_digits.push(n) 
            }
        });

        dbg!((number_digits.get(0).unwrap_or(&0) * 10) + (number_digits.get(number_digits.len() -1).unwrap_or(&0)));

        total += (number_digits.get(0).unwrap_or(&0) * 10) + (number_digits.get(number_digits.len() -1).unwrap_or(&0));
    });

    total
}
