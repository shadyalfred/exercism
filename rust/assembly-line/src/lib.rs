// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0 => 0_f64,
        1..=4 => 1_f64,
        5..=8 => 0.9_f64,
        9..=10 => 0.77_f64,
        _ => panic!("Invalid speed"),
    };

    (speed as u32 * 221) as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60_f64) as u32
}
