use std::{fs, collections::HashMap};
use indexmap::IndexMap;

#[derive(Debug)]
struct InitSequence {
    steps: Vec<String>
}

impl InitSequence {
    fn build(input: &str) -> InitSequence {
        let steps = input.split(",").map(|val| val.to_owned()).collect();
        InitSequence { steps }
    }

    fn calculate_result(&self) -> u32 {
        self.steps.iter().map(|value| {
            hash(value)
        }).sum()
    }

    fn init_lenses(&self) -> HashMap<u32, IndexMap<String, u32>> {
        let mut boxes: HashMap<u32, IndexMap<String, u32>> = HashMap::new();

        self.steps.iter().for_each(|step| {
            if step.contains("=") {
                let (label, focal_str) = step.split_once('=').unwrap();
                let focal_len: u32 = focal_str.parse().unwrap();
                let lens_map: &mut IndexMap<String, u32> = boxes.entry(hash(&label)).or_insert(IndexMap::new());
                lens_map.insert(label.to_owned(), focal_len);
            } else {
                let label = &step.as_str()[0..step.len()-1];
                let lens_map: &mut IndexMap<String, u32> = boxes.entry(hash(&label)).or_insert(IndexMap::new());
                lens_map.shift_remove(label);
            }
        });

        boxes
    }
}

fn hash(inp: &str) -> u32 {
    let mut total = 0;

    for c in inp.chars() {
        total += c as u32;
        total *= 17;
        total = total % 256;
    }

    total
}

fn main() {
    let mut input = fs::read_to_string("input").unwrap();
    let input_len = input.len();
    input.truncate(input_len -1);
    let init = InitSequence::build(&input);
    let boxes = init.init_lenses();
    let mut focusing_power = 0;

    boxes.keys().for_each(|box_number| {
        let val = boxes.get(box_number).unwrap();
        val.keys().for_each(|label| {
            focusing_power += (box_number+1) * (val.get_index_of(label).unwrap() as u32 + 1) * val.get(label).unwrap();
        });
    });

    println!("{}", focusing_power);
}
