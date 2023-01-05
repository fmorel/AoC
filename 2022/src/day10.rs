use crate::file_utils::file_to_lines;
use std::path::Path;

pub fn day10(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut register: i32 = 1;
    let mut register_hist: Vec<i32> = Vec::new();    /* Register value for each cycle */

    for l in lines {
        let tokens: Vec<&str> = l.split_ascii_whitespace().collect();
        match tokens[0] {
            "noop" => register_hist.push(register),
            "addx" => {
                        register_hist.push(register);
                        register_hist.push(register);
                        register += tokens[1].parse::<i32>().unwrap();
                      }
            s => panic!("Unexpected token {}", s)
        }
    }
    let mut signal_strength_sum = 0;
    /* Part 1 */
    for i in 0..6 {
        let cycle = 20 +  i*40;
        let signal_strength = register_hist[cycle-1] * (cycle as i32);
        signal_strength_sum += signal_strength;
        println!("Signal strength at cycle {} is {}, sum is {}", cycle, signal_strength, signal_strength_sum);
    }
    /* Part 2 */
    for i in 0..6 {
        let mut line = String::new();
        for j in 0..40 {
            let cycle = i*40 + j;
            let r = register_hist[cycle];
            if r-(j as i32) >= -1 && r-(j as i32) <= 1 {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}
