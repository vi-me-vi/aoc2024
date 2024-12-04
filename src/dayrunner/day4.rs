use alloc::{
    format,
    string::String,
    vec::Vec,
    collections::BTreeMap
};
use crate::aoc_utils::logging;


pub fn d4_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day4/run.log"));
    let lines: Vec<String> = input.lines().map(|line| String::from(line.trim())).collect();

    // PART 1
    let mut res1 = 0;
    let sw = "XMAS";
    let swr = "SAMX";

    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'X' || c == 'S' {
                let check: &str;
                if c == 'X' { check = sw } else { check = swr }

                // HORIZONTAL CHECK
                if j <= l.len() - check.len() {
                    if &l[j..j+check.len()] == check {
                        res1 += 1;
                    }
                }

                // VERTICAL CHECKS
                if i <= lines.len() - check.len() {
                    let mut down = 1;
                    let mut front_diagonal = if j <= l.len() - check.len() { 1 } else { 0 };
                    let mut back_diagonal = if j >= check.len()-1 { 1 } else { 0 };
                    for y in 1..check.len() {
                        let c_check = check.chars().nth(y).unwrap();

                        // FRONT DIAGONAL CHECK
                        if front_diagonal > 0 && lines[i+y].chars().nth(j+y).unwrap() != c_check {
                            front_diagonal = 0;
                        }

                        // VERTICAL CHECK
                        if down > 0 && lines[i+y].chars().nth(j).unwrap() != c_check {
                            down = 0;
                        }

                        // BACK DIAGONAL CHECK
                        if back_diagonal > 0 && lines[i+y].chars().nth(j-y).unwrap() != c_check {
                            back_diagonal = 0;
                        }
                    }
                    res1 += down + front_diagonal + back_diagonal;
                }
            }
        }
    }
    logger.log(&format!("[day4] part one: {:?}", res1));

    // PART 2
    let mut res2 = 0;
    let edge_map = BTreeMap::from([
        ('M', 'S'),
        ('S', 'M'),
    ]);

    for (i, l) in lines[1..lines.len()-1].iter().enumerate() {
        for (j, c) in l[1..l.len()-1].chars().enumerate() {
            if c == 'A' {
                // NOTE: indexing of i and j is shifted by 1, because we enumerate slice
                match edge_map.get(&lines[i].chars().nth(j).unwrap()) {
                    Some(val) => if val != &lines[i+2].chars().nth(j+2).unwrap() { continue },
                    None => continue,
                }
                match edge_map.get(&lines[i].chars().nth(j+2).unwrap()) {
                    Some(val) => if val != &lines[i+2].chars().nth(j).unwrap() { continue },
                    None => continue,
                }
                res2 += 1
            }
        }
    }

    logger.log(&format!("[day4] part two: {:?}", res2));


    format!("Day 4:\n    PART 1: {:#?}\n    PART 2: {}\n", res1, res2)
}
