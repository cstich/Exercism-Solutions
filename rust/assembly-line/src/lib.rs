// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0
    };
    f64::from(speed) * success_rate * RATE
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let x = production_rate_per_hour(speed) / 60.0;
    x as u32
}
