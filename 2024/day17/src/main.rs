use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    dbg!(part1(&input));
}


fn part1(input: &str) -> u64 {
    let (regs_str, program_str) = input.split_once("\n\n").unwrap();
    let mut regs: Vec<u64> = regs_str.lines().map(|r| r.split(" ").last().unwrap().parse::<u64>().unwrap()).collect();
    let program: Vec<u8> = program_str.split(":").last().unwrap().trim().split(",").map(|i| i.parse::<u8>().unwrap()).collect();

    dbg!(&regs);
    dbg!(&program);

    let mut pc = 0;

    
    for A in 0..u64::MAX {
        regs[0] = A;
        let mut output: Vec<u64> = Vec::new();

        if A % 100000000 == 0 {
            dbg!(A);
        }

        'programloop: while pc < program.len() {
            let operand = program[pc+1];
            match program[pc] {
                0 => regs[0] = regs[0] / u64::pow(2, combo_val(operand, &regs) as u32),
                1 => regs[1] = regs[1] ^ operand as u64,
                2 => regs[1] = combo_val(operand, &regs) % 8,
                3 => if regs[0] != 0 { pc = operand as usize; continue 'programloop; },
                4 => regs[1] = regs[1] ^ regs[2],
                5 => {
                    print!("{},", combo_val(operand, &regs) % 8);


                },
                6 => regs[1] = regs[0] / u64::pow(2, combo_val(operand, &regs) as u32),
                7 => regs[2] = regs[0] / u64::pow(2, combo_val(operand, &regs) as u32),
                _ => panic!(),
            };
            pc+=2;
        }

        let out: Vec<u8> = output.iter().map(|n| *n as u8).collect();
        if out == program {
            return A;
        }

    }

    0
}

fn combo_val(operand: u8, regs: &Vec<u64>) -> u64 {
    match operand {
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        default => default as u64
    }
}

fn part2(input: &str) {
    todo!()
}
