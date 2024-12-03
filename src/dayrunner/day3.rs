use alloc::{format, string::String, vec::Vec};
use crate::aoc_utils::logging;


pub fn d3_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day3/run.log"));

    // PART 1
    let mut res1: i32 = 0;
    let split1 = input.split("mul(");
    for m in split1 {
        let hold: Vec<&str>= match m.split(")").next() {
            Some(val) => val.split(",").collect(),
            None => continue,
        };
        if hold.len() == 2 {
            let a = match hold[0].parse::<i32>() {
                Ok(val) => val,
                Err(_) => continue,
            };
            let b = match hold[1].parse::<i32>() {
                Ok(val) => val,
                Err(_) => continue,
            };
            res1 += a * b;
        }
    }

    logger.log(&format!("[day3] part one: {:?}", res1));

    // PART 2
    let mut res2: i32 = 0;
    let split2_1: Vec<&str> = input.split("don't()").collect();
    for (i, l) in split2_1.iter().enumerate() {
        let line: &str;
        if i == 0 {
            line = l;
        } else {
            line = match l.split_once("do()") {
                Some(val) => val.1,
                None => continue,
            }
        }
        let split2_2 = line.split("mul(");
        for m in split2_2 {
            let hold: Vec<&str> = match m.split(")").next() {
                Some(val) => val.split(",").collect(),
                None => continue,
            };
            if hold.len() == 2 {
                let a = match hold[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let b = match hold[1].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                res2 += a * b;
            }
        }
    }
    logger.log(&format!("[day3] part two: {:?}", res2));


    format!("Day 2:\n    PART 1: {:#?}\n    PART 2: {:#?}\n", res1, res2)
}
