use alloc::{format, string::String, vec::Vec};
use crate::aoc_utils::logging;


pub fn d3_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day3/run.log"));

    // PART 1
    let mut res1: i32 = 0;
    let split1 = input.split("mul(");
    for m in split1 {
        if !m.contains(")") {
            continue;
        }
        let hold: Vec<&str>;
        match m.split(")").next() {
            Some(val) => hold = val.split(",").collect(),
            None => return String::from("Error, wrong mul() arguments in input"),
        }
        if hold.len() == 2 && hold[0].parse::<i32>().is_ok() && hold[1].parse::<i32>().is_ok() {
            res1 += hold[0].parse::<i32>().unwrap() * hold[1].parse::<i32>().unwrap();
        }
    }

    logger.log(&format!("[day2] part one: {:?}", res1));

    // PART 2
    let mut res2: i32 = 0;
    let split2_1: Vec<&str> = input.split("don't()").collect();
    for (i, l) in split2_1.iter().enumerate() {
        let line: &str;
        if i == 0 {
            line = l;
        } else if l.contains("do()") {
            match l.split_once("do()") {
                Some(val) => line = val.1,
                None => return String::from(*l),
            }
        } else {
            continue;
        }
        let split2_2 = line.split("mul(");
        for m in split2_2 {
            if !m.contains(")") {
                continue;
            }
            let hold: Vec<&str>;
            match m.split(")").next() {
                Some(val) => hold = val.split(",").collect(),
                None => return String::from("Error, wrong mul() arguments in input"),
            }
            if hold.len() == 2 && hold[0].parse::<i32>().is_ok() && hold[1].parse::<i32>().is_ok() {
                res2 += hold[0].parse::<i32>().unwrap() * hold[1].parse::<i32>().unwrap();
            }
        }
    }
    logger.log(&format!("[day2] part two: {:?}", res2));


    format!("Day 2:\n    PART 1: {:#?}\n    PART 2: {:#?}\n", res1, res2)
}
