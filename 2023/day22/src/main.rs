use std::{fs, collections::{HashMap, VecDeque, HashSet}, process::exit, io};

#[derive(Debug)]
struct Brick {
    startx: u32,    
    starty: u32,    
    startz: u32,    
    endx: u32,    
    endy: u32,    
    endz: u32,
}
impl Brick {
    fn build(line: &str) -> Brick {
        let (start, end) = line.split_once("~").unwrap();

        let mut start_iter = start.splitn(3, ",");
        let startx: u32 = start_iter.next().unwrap().parse().unwrap();
        let starty: u32 = start_iter.next().unwrap().parse().unwrap();
        let startz: u32 = start_iter.next().unwrap().parse().unwrap();

        let mut end_iter = end.splitn(3, ",");
        let endx: u32 = end_iter.next().unwrap().parse().unwrap();
        let endy: u32 = end_iter.next().unwrap().parse().unwrap();
        let endz: u32 = end_iter.next().unwrap().parse().unwrap();

        Brick {
            startx,
            starty,
            startz,
            endx,
            endy,
            endz,
        }
    }
}

struct TopDown {
    grid: HashMap<(u32, u32), u32>
}


fn main() {
    let input = fs::read_to_string("sample").unwrap();

    let bricks: Vec<Brick> = input.lines().map(|line| Brick::build(line)).collect();

    dbg!(bricks);
}
