#![allow(unused_imports)]
use std::path::Path;

/* Create modules */
mod file_utils;
mod day1_to_4;
use day1_to_4::{day_1, day_2, day_3, day_4};

/* Main */
fn main()
{
    //day_1(Path::new("inputs/day1.txt"));
    //day_2(Path::new("inputs/day2.txt"));
    //day_3(Path::new("inputs/day3.txt"), true);
    day_4(Path::new("inputs/day4.txt"), true);
}
