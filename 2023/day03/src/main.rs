use std::fs;
use itertools::Itertools;


#[derive(Debug, Copy, Clone)]
enum EngineNode {
    SYMBOL {symbol: char, pos: (usize, usize)},
    NUMBER {num: u32, start_pos: (usize, usize), len: usize},
    EMPTY,
}

#[derive(Debug)]
struct EngineSchematic {
    canvas: Vec<Vec<EngineNode>>
}


impl EngineSchematic {
    fn build(input: &str) -> EngineSchematic {
        let canvas: Vec<Vec<EngineNode>> = input.lines().enumerate().map(|(y_idx, line)| {
            let mut engine_line: Vec<EngineNode> = Vec::new();
            let mut seek_to_idx = 0; 

            for (x_idx, char) in line.chars().enumerate() {
                if x_idx < seek_to_idx {
                    continue;
                }
                match char {
                    '.' => engine_line.push(EngineNode::EMPTY),
                    '0'..='9' => {
                        let num_len = get_num_size(line.to_owned().chars().collect(), x_idx);
                        seek_to_idx = x_idx+num_len;
                        let num = line[x_idx..x_idx+num_len].to_owned().parse::<u32>().unwrap();
                        let node = EngineNode::NUMBER {
                            num,
                            start_pos: (x_idx, y_idx),
                            len: num_len,
                        };
                        for _ in 0..num_len {
                            engine_line.push(node)
                        };
                    },
                    symbol => engine_line.push(EngineNode::SYMBOL {
                        symbol,
                        pos: (x_idx, y_idx)
                    })
                };
            };
            engine_line
        }).collect();
    
        EngineSchematic { canvas }

    }

    fn get(&self, pos: (i32, i32)) -> EngineNode {
        let x_pos = usize::try_from(pos.0);
        let y_pos = usize::try_from(pos.1);
        
        match (x_pos, y_pos) {
            (Ok(x), Ok(y)) => {
                if let Some(row) = self.canvas.get(y) {
                    row.get(x).unwrap_or(&EngineNode::EMPTY).to_owned()
                } else {
                    EngineNode::EMPTY
                }
            },
            _ => EngineNode::EMPTY
        }
    }

    fn get_symbols(&self) -> Vec<(char, (usize, usize))> {
        self.canvas.iter().flatten().filter_map(|node| {
            if let EngineNode::SYMBOL { symbol, pos } = node {
                Some((symbol.to_owned(), pos.to_owned()))
            } else {
                None
            }
        }).collect()
    }

}

fn get_num_size(canvas: Vec<char>, idx: usize) -> usize {
    if let None = canvas.get(idx).unwrap_or(&'.').to_digit(10) {
        return 0
    }
    get_num_size(canvas, idx+1) + 1
}

fn get_adjacent_indexes(pos: (usize, usize)) -> std::vec::IntoIter<(i32, i32)> {
    let x = i32::try_from(pos.0).unwrap();
    let y = i32::try_from(pos.1).unwrap();
    let adj = vec![(x, y -1,), (x -1, y -1), (x -1, y), (x -1, y +1), (x, y +1), (x +1, y +1), (x + 1, y), (x +1, y -1)];

    adj.into_iter()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let engine = EngineSchematic::build(&input);

    println!("{}", part1(&engine));

}

fn part1(engine: &EngineSchematic) -> u32 {
    let mut total = 0;

    for (_, pos) in engine.get_symbols() {
        
       total += get_adjacent_indexes(pos).filter_map(|adj_pos| {
            if let EngineNode::NUMBER {num, start_pos, len} = engine.get(adj_pos) {
                Some((num, start_pos))
            } else {
                None
            }
        }).unique().map(|(num, _start_pos)| num).sum::<u32>();
    }
    total
}

