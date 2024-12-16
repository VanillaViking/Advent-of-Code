use std::fs;

fn main() {
    let input = fs::read_to_string("sample").unwrap();
    let reports: Vec<Vec<u32>> = input.lines().map(|line| line.split(" ").map(|n| n.parse::<u32>().unwrap()).collect()).collect();
    dbg!(part1(&reports));
}

fn part1(reports: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    let mut omits = 1;
    'reports: for report in reports.iter() {
        let mut diffs = Vec::new();
        for idx in 1..report.len() {
            diffs.push(report[idx -1] as i64 - report[idx] as i64);

        }
        
        let mut neg = false;
        if diffs[0] < 0 {
            neg = true;
        }
        for idx in 0..diffs.len() {
            if diffs[idx] == 0 {
                if try_omit(omits, &mut diffs, idx, neg) {
                    continue;
                } else {
                    continue 'reports;
                }
            }
            if neg {
                if diffs[idx] > 0 || diffs[idx] < -3 {
                    if try_omit(omits, &mut diffs, idx, neg) {
                        continue;
                    } else {
                        continue 'reports;
                    }
                }
            } else {
                if diffs[idx] < 0 || diffs[idx] > 3 {
                    if try_omit(omits, &mut diffs, idx, neg) {
                        continue;
                    } else {
                        continue 'reports;
                    }
                }
            }

        }

        count+=1;
    }
    
    count
}

fn try_omit(omits: i32, diffs: &mut Vec<i64>, idx: usize, neg: bool) -> bool {
    if omits <= 0 {
        return false;
    }

    if (diffs[idx-1] - diffs[idx+1] < 0) == neg {
        if (diffs[idx-1] - diffs[idx+1]).abs() != 0 && (diffs[idx-1] - diffs[idx+1]).abs() < 3 {
            diffs.remove(idx);
            return true;
        }
    }

    if let Some(prev) = idx.checked_sub(2) {
        if (prev - diffs[idx] < 0) == neg {
            if (prev - diffs[idx]).abs() != 0 && (prev - diffs[idx]).abs() < 3 {
                diffs.remove(idx-1);
                return true;
            }
        }
    } else {
        return true;
    }

    return false;

}
