/*
 * valid: mul(X,Y)
 *
 * test if str[n,n+3] == "mul"
 *
 *
 * 
 *
 * */

use std::fs;

fn main() {
    let mut input = fs::read_to_string("input").unwrap();
    
    let mut total: u64 = 0;
    let mut mul_toggle = true;

    for n in 0..input.len()-3 {
        if n+7 < input.len() {
            if &input[n..n+7] == "don't()" {
                mul_toggle = false;
                continue;
            }
        }

        if n+4 < input.len() {
            if &input[n..n+4] == "do()" {
                mul_toggle = true;
                continue;
            }
        }
        
        if !mul_toggle {
            continue;
        }

        if &input[n..n+3] == "mul" {
            if let Some(res) = multiply(&input, n+4) {
                total += res;
            }
        }
    }

    dbg!(total);
}

fn multiply(input: &str, idx: usize) -> Option<u64> {
    let end = input[idx..].find(")")? + idx;
    let parse_str = &input[idx..end];

    let (a, b) = parse_str.split_once(",")?;

    Some(a.parse::<u64>().ok()? * b.parse::<u64>().ok()?)
}
