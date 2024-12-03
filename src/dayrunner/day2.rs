use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::aoc_utils::logging;


pub fn d2_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day2/run.log"));
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut lines: Vec<String>;
    lines = input.lines().map(|line| line.trim().to_string()).collect();


    while let Some(line) = lines.pop() {
        let mut line_split = line.split_whitespace();
        // TODO: needs map here
        let mut nvec: Vec<u32> = Vec::new();
        while let Some(num) = line_split.next() {
            nvec.push(num.parse().unwrap());
        }
        matrix.push(nvec);
    }

    // PART 1
    let mut res1 = 0;
    for l in &matrix {
        if l.is_sorted_by(|x, y| {x > y && x.abs_diff(*y) < 4}) || l.is_sorted_by(|x, y| {x < y && x.abs_diff(*y) < 4}) {
            res1 += 1;
        }
    }
    logger.log(&format!("[day2] part one: {:?}", res1));

    // PART 2 TODO: can be improved by inteligently skipping to error part and checking there
    let mut res2 = 0;
    'outer: for l in &matrix {
        if l.is_sorted_by(|x, y| {x > y && x.abs_diff(*y) < 4}) || l.is_sorted_by(|x, y| {x < y && x.abs_diff(*y) < 4}) {
            res2 += 1;
            continue;
        }
        for i in 0..l.len() {
            // NOTE: Retain by id due to duplicated entries
            let l_c: Vec<u32> = l
                .into_iter()
                .enumerate()
                .filter(|&(id, _)| id != i).map(|(_, v)| *v)
                .collect();

            if l_c.is_sorted_by(|x, y| {x > y && x.abs_diff(*y) < 4}) || l_c.is_sorted_by(|x, y| {x < y && x.abs_diff(*y) < 4}) {
                res2 += 1;
                continue 'outer;
            }
        }
    }
    logger.log(&format!("[day2] part two: {:?}", res2));


    format!("Day 2:\n    PART 1: {:#?}\n    PART 2: {:#?}\n", res1, res2)
}
