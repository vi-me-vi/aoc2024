use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::aoc_utils::logging;


pub fn d1_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day1/run.log"));

    // PARSE INPUT
    let mut l_vec: Vec<i32> = Vec::new();
    let mut r_vec: Vec<i32> = Vec::new();
    {
        let mut lines_vec: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

        // TODO: map here
        while let Some(line) = lines_vec.pop() {
            let mut line_split = line.split_whitespace();
            match line_split.next() {
                Some(val) => {
                    l_vec.push(val.parse().unwrap());
                },
                None => return "Error parsing".to_string(),
            }
            match line_split.next() {
                Some(val) => {
                    r_vec.push(val.parse().unwrap());
                },
                None => return "Error parsing".to_string(),
            }
        }
    }
    logger.log(&format!("[day1] parsing completed"));

    // // PART 1
    let mut res1 = 0;

    l_vec.sort();
    r_vec.sort();

    for i in 0..l_vec.len() {
        res1 += (l_vec[i] - r_vec[i]).abs()
    }
    logger.log(&format!("[day1] part one: {}", res1));

    // PART 2 TODO: can be improved by caching in hash map
    let mut res2: i32 = 0;

    for l in &l_vec {
        res2 += l * (r_vec.iter().filter(|&&x| x == *l).count() as i32);
    }
    logger.log(&format!("[day1] part two: {}", res2));


    format!("Day 1:\n    PART 1: {:#?}\n    PART 2: {:#?}\n", res1, res2)
}
