use std::fs;


#[derive(Debug)]
struct EngineSchematic {
    size: i32,
    canvas: Vec<char>,
    //                     num  start  len
    number_positions: Vec<(u32, i32, i32)>,
    gear_positions: Vec<usize>
}

impl EngineSchematic {
    fn get(&self, idx: usize) -> char {
        self.canvas.get(idx).unwrap_or(&'.').to_owned()
    }

    fn is_symbol(&self, idx: i32) -> bool {
        if let Ok(n) = usize::try_from(idx) {
            return self.get(n) != '.' && !self.get(n).is_digit(10)
        };
        return false
    }

    fn is_digit(&self, idx: i32) -> bool {
        if let Ok(n) = usize::try_from(idx) {
            return self.get(n).is_digit(10)
        };
        return false
    }

    pub fn get_num_size(&self, idx: usize) -> usize {
        if let None = self.get(idx).to_digit(10) {
            return 0
        }
        self.get_num_size(idx+1) + 1
    }


    pub fn get_part_numbers_sum(&self) -> u32 {
        let mut total = 0;

        //hacky
        let mut seek_to_idx = 0;

        for (idx, _char) in self.canvas.iter().enumerate() {
            if idx < seek_to_idx {
                continue;
            }

            let num_size = self.get_num_size(idx);
            if num_size == 0 {
                continue
            } else {
                if let Ok(n) = i32::try_from(idx) {
                    if self.check_adjacent(n) {
                        let num = self.canvas[idx..idx+num_size].iter().collect::<String>().parse::<u32>().unwrap();
                        total += num;

                    }
                }
                seek_to_idx = idx+num_size
            }
        }

        total
    }

    fn check_adjacent(&self, idx: i32) -> bool {
        // checking indexes behind the starting num: 
        //          -->* * * *
        // these 3: -->* 2 3 *
        //          -->* * * *
        let adj_behind: Vec<i32> = vec![idx-1, idx-self.size-1, idx+self.size-1];
        for idx in adj_behind {
            if self.is_symbol(idx) {
                return true
            }
        }

        // check indexes on top & below numbers recursively
        self.number_adjacents(idx)
    }

    fn number_adjacents(&self, idx: i32) -> bool {
        // top
        if self.is_symbol(idx - self.size) {
            return true
        }
        // bottom
        if self.is_symbol(idx + self.size) {
            return true
        }

        if self.is_symbol(idx) {
            return true
        } else if !self.is_digit(idx) {
            return false
        }

        self.number_adjacents(idx+1)
    }

    pub fn get_gear_ratio_sum(&self) -> u32 {

        self.gear_positions.iter().filter_map(|idx| {
            self.check_surroundings(i32::try_from(idx.to_owned()).unwrap())
        }).map(|(num1, num2)| num1 * num2).sum()
    }

    fn check_surroundings(&self, idx: i32) -> Option<(u32, u32)> {
        let adj: Vec<i32> = vec![idx-self.size, idx-self.size-1, idx-1, idx+self.size-1, idx+self.size, idx+self.size+1, idx+1, idx-self.size+1];

        let adjacent_numbers: Vec<u32> = self.number_positions.iter().filter_map(|(num, start, len)| {
            for adj_idx in adj.iter() {
                if *adj_idx >= *start && *adj_idx < start + len {
                    return Some(num.to_owned())
                }
            }
            return None
        }).collect();
        
        if adjacent_numbers.len() == 2 {
            Some((adjacent_numbers[0], adjacent_numbers[1]))
        } else {
            None
        }
    }
}


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let sample = fs::read_to_string("sample").unwrap();

    let engine = parse_engine(&input);

    println!("{}", engine.get_part_numbers_sum());
    println!("{}", engine.get_gear_ratio_sum());

}


fn get_num_size(canvas: Vec<char>, idx: usize) -> usize {
    if let None = canvas.get(idx).unwrap_or(&'.').to_digit(10) {
        return 0
    }
    get_num_size(canvas, idx+1) + 1
}

fn parse_engine(input: &str) -> EngineSchematic  {
    let binding = input.replace("\n", "");
    let canvas = binding.chars();
    let mut number_positions: Vec<(u32, i32, i32)> = Vec::new();

     let mut seek_to_idx = 0;


    for (idx, char) in canvas.to_owned().enumerate() {
        if idx < seek_to_idx {
            continue;
        }

        if !char.is_digit(10) {
            continue
        } else {
            let num_len = get_num_size(canvas.to_owned().collect(), idx);
            seek_to_idx = idx+num_len;
            let num = canvas.to_owned().collect::<Vec<char>>()[idx..idx+num_len].iter().collect::<String>().parse::<u32>().unwrap();
            number_positions.push((num ,i32::try_from(idx).unwrap(), i32::try_from(num_len).unwrap()));
        }
    }

    EngineSchematic { 
        size: 140, 
        canvas: canvas.to_owned().collect(),
        gear_positions: canvas.enumerate().filter_map(|(idx, char)| {
            if char == '*' {
                Some(idx)
            } else {
                None
            }
        }).collect(),
        number_positions
    }

}

