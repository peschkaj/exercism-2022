// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_PRODUCTION_PER_HOUR:u8 = 221;

fn success_rate(speed:u8) -> f64 {
    match speed {
        x if x >= 1 && x <= 4 => 1.0,
        x if x > 4 && x <= 8 => 0.9,
        x if x > 8 && x <= 10 => 0.77,
        _ => 0.0
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production = (speed as i32 * BASE_PRODUCTION_PER_HOUR as i32) as f64;
    production * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hourly = production_rate_per_hour(speed);
    (hourly / 60 as f64) as u32
}
