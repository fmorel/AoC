use std::path::Path;

/* Create modules */
mod file_utils;
/*
mod day1_to_4;
mod day5;
mod day6;
mod day7;
mod day8;
use day1_to_4::{day_1, day_2, day_3, day_4};
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
mod day9;
use day9::day9;
mod day12;
use day12::day12;
*/
mod day10;
use day10::day10;

/* Main */
fn main()
{
    //day_1(Path::new("inputs/day1.txt"));
    //day_2(Path::new("inputs/day2.txt"));
    //day_3(Path::new("inputs/day3.txt"), true);
    //day_4(Path::new("inputs/day4.txt"), true);
    //day5(Path::new("inputs/day5.txt"), true);
    //day6(Path::new("inputs/day6.txt"), true);
    //day7(Path::new("inputs/day7.txt"));
    //day8(Path::new("inputs/day8.txt"));
    //day9(Path::new("inputs/day9.txt"));
    //day12(Path::new("inputs/day12.txt"), true);
    day10(Path::new("inputs/day10.txt"));
}
