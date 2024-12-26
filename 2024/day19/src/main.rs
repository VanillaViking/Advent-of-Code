use std::{cmp, collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (available_str, patterns_str) = input.split_once("\n\n").unwrap();
    
    let mut available = HashSet::new();
    
    let mut largest = 0;
    available_str.split(", ").for_each(|towel| {
        available.insert(towel.to_owned());
        largest = cmp::max(largest, towel.len());
    });

    let patterns: Vec<String> = patterns_str.lines().map(|s| s.to_owned()).collect();
    
    dbg!(largest);
    dbg!(&available);
    dbg!(&patterns);

    part1(&available, &patterns, largest);
    part2(&available, &patterns, largest);

}

fn part1(available: &HashSet<String>, patterns: &Vec<String>, largest: usize) {
    let passed: Vec<bool> = patterns.iter()
        .map(|pat| find_possible(available, pat, largest, 0)).collect();


    dbg!(&passed);
    dbg!(passed.iter().filter(|p| **p)
        .count());

}

fn part2(available: &HashSet<String>, patterns: &Vec<String>, largest: usize) {
    dbg!(patterns.iter().map(|pat| { 
        let mut cache = HashMap::new();
        dbg!(numwaysusing_tomake_withstart(available, pat, 0, largest, &mut cache))
    }).sum::<u64>());

}

fn numwaysusing_tomake_withstart(available: &HashSet<String>, pattern: &str, start_idx: usize, largest: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if let Some(s) = cache.get(&start_idx) {
        return s.clone() as u64;
    }
    if start_idx >= pattern.len() {
        return 1;
    }

    let mut ways: u64 = 0;
    for end in (1..=largest).rev() {
        if start_idx+end > pattern.len() {
            continue;
        }
        if available.contains(&pattern[start_idx..(start_idx+end)]) {
            ways += numwaysusing_tomake_withstart(available, pattern, start_idx+end, largest, cache);
        }
    }
    cache.insert(start_idx, ways);
    ways
}

fn find_possible(available: &HashSet<String>, pattern: &str, largest: usize, start_idx: usize) -> bool {

    if start_idx >= pattern.len() {
        return true;
    }

    for end in (1..=largest).rev() {
        if start_idx+end > pattern.len() {
            continue;
        }
        if available.contains(&pattern[start_idx..(start_idx+end)]) {
            if find_possible(available, pattern, largest, start_idx+end) {
                return true;
            }
        }
    }
    false

}
