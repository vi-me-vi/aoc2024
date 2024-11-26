use alloc::string::{String, ToString};
use alloc::format;
use crate::aoc_utils::{read, logging};


pub fn d1_run() -> String {
    // let res = 0;
    let logger = logging::AoCLogger::new(String::from("./day1.log"));

    let read_str = read::into_str("./day1.in");
    logger.log(&read_str);

    let lines_vec = read::into_lines_vec("./day1.in");
    logger.log(&format!("{:#?}", lines_vec).to_string());

    let char_matrix = read::into_char_metrix("./day1.in");
    logger.log(&format!("{:#?}", char_matrix).to_string());

    format!("Day 1:\n{}\n", read_str).to_string()
}
