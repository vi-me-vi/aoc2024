use core::cmp::Ordering;
use alloc::{format, string::String, vec::Vec, collections::BTreeMap};
use crate::aoc_utils::logging;


pub fn d5_run(input: String) -> String {
    let logger = logging::AoCLogger::new(String::from("./day5/run.log"));

    let mut split = input.split("\n\n");
    let rules: Vec<&str> = match split.next() {
        Some(val) => val.split("\n").collect(),
        None => return String::from("Error parsing rules"),
    };
    let updates: Vec<&str> = match split.next() {
        Some(val) => val.trim().split("\n").collect(),
        None => return String::from("Error parsing updates"),
    };
    let mut mapper: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for p in &rules {
        let mut pair_split = p.split("|");
        let pref: &str = match pair_split.next() {
            Some(val) => val,
            None => return String::from("Error parsing a rule pair"),
        };
        let key: &str = match pair_split.next() {
            Some(val) => val,
            None => return String::from("Error parsing a rule pair"),
        };
        mapper.entry(key)
            .or_insert_with(Vec::new)
            .push(pref);
        mapper.entry(pref)
            .or_insert_with(Vec::new);
    }

    // PART 1
    let mut res1: i32 = 0;
    for up in &updates {
        let sup: Vec<&str> = up.split(",").collect();
        if sup.is_sorted_by(|a, b| mapper[b].contains(a)) {
            res1 += sup[sup.len()/2].parse::<i32>().unwrap();
        }
    }
    logger.log(&format!("[day5] part two: {}", res1));

    //PART 2
    let mut res2: i32 = 0;
    for up in &updates {
        let mut sup: Vec<&str> = up.split(",").collect();
        if !sup.is_sorted_by(|a, b| mapper[b].contains(a)) {
            // NOTE: order doesn't matter here really, if we have guaranteed single middle element
            sup.sort_by(|a, b| match mapper[b].contains(a) {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
            res2 += sup[sup.len()/2].parse::<i32>().unwrap();
        }
    }
    logger.log(&format!("[day5] part two: {}", 0));


    format!("Day 5:\n    PART 1: {:#?}\n    PART 2: {:#?}\n", res1, res2)
}
