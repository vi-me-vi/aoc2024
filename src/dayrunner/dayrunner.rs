use alloc::string::String;
use super::{
    day1::d1_run,
    day2::d2_run,
    day3::d3_run,
    day4::d4_run
};


static DAY_MAPPER: [fn(String) -> String; 4] = [
    d1_run, d2_run, d3_run, d4_run,
];

pub fn run_day(day_index: i8, input: String) -> String {
    if let Some(runner) = DAY_MAPPER.get(day_index as usize) {
        runner(input)
    } else {
        String::from("This day is not implemented...")
    }
}
