use std::fs;

#[derive(Debug)]
struct Row {
    records: String,
    groups: Vec<u32>,
}

impl Row {
    pub fn new(line: &str) -> Row {
        let (records, groups_str) = line.split_once(" ").unwrap();
        let groups: Vec<u32> = groups_str.split(",").map(|num_str| num_str.parse::<u32>().unwrap()).collect();

        Row { records: records.to_owned(), groups }
    }
}

fn main() {
    let input = fs::read_to_string("sample").unwrap();
    let springs: Vec<Row> = input.lines().map(|line| Row::new(line)).collect();
    dbg!(&springs);
    
    let part1: u32 = springs.iter().map(|row| {
        count_arrangements(row)
    }).sum();
}

fn count_arrangements(row: &Row) -> u32 {

}
