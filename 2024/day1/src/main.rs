use std::collections::HashMap;
use std::{collections::BinaryHeap, fs};
use std::cmp::{self, Reverse};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    let mut map2: HashMap<u32,u32> = HashMap::new();

    for line in input.lines() {
        let (num1_str,num2_str) = line.split_once(" ").unwrap();
        vec1.push(num1_str.parse().unwrap());
        vec2.push(num2_str.parse().unwrap());
    }

    for n in vec2.iter() {
        map2.entry(n.clone()).and_modify(|n| *n+=1).or_insert(1);
    }

    let mut total = 0;
    for x in vec1.iter() {
        total += x * map2.get(x).unwrap_or(&0);

    }

    dbg!(total);
}
