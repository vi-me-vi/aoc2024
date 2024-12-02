use alloc::string::String;
use super::{
    day1::d1_run,
    day2::d2_run,
};


static DAY_MAPPER: [fn(String) -> String; 2] = [
    d1_run, d2_run,
];

pub fn run_day(day_index: i8, input: String) -> String {
    if let Some(runner) = DAY_MAPPER.get(day_index as usize) {
        runner(input)
    } else {
        String::from("This day is not implemented...")
    }
}
