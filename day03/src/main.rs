use std::fs;


#[derive(Debug)]
struct EngineSchematic {
    size: i32,
    pub canvas: Vec<char>
}

impl EngineSchematic {
    // pub fn check_adjacent(&self, idx: usize) { 
    //     let adj: Vec<usize> = vec![idx-self.size, idx-self.size-1, idx-1, idx+self.size-1, idx+self.size, idx+self.size+1, idx+self.size, idx-self.size+1];

    //     let adj_numbers = adj.iter().filter_map(|adj_idx| {
    //         let a = self.canvas.get(adj_idx.to_owned()).unwrap_or(&'.');
    //         0
    //     });
    // }
    //
    
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

        for (idx, char) in self.canvas.iter().enumerate() {
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
                        dbg!(num);
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
}



fn main() {

    let input = fs::read_to_string("input").unwrap();
    let sample = fs::read_to_string("sample").unwrap();

    let engine = parse_engine(&input);

    println!("{}", engine.get_part_numbers_sum());

    // println!("{}", part1());
    println!("{}", part2(&input));
}

fn parse_engine(input: &str) -> EngineSchematic  {
    EngineSchematic { size: 140, canvas: input.replace("\n", "").chars().collect() }

}



fn part2(input: &str) -> u32 {
    0
}
