use alloc::string::{String, ToString};
use super::day1::d1_run;


static DAY_MAPPER: [fn() -> String; 1] = [d1_run];

pub fn run_day(day_index: i8) -> String {
    if let Some(runner) = DAY_MAPPER.get(day_index as usize) {
        runner()
    } else {
        "This day is not implemented...".to_string()
    }
}
