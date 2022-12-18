#![allow(dead_code)]

use crate::file_utils::file_to_lines;
use std::path::Path;

fn parse_initial_stacks(stacks: &mut [Vec<u8>], line: &str) -> bool
{
    let chars = line.as_bytes();
    let mut i = 1;
    let mut header = true;
    for s in stacks.iter_mut() {
        match chars[i] {
            b'A'..=b'Z' => s.insert(0, chars[i]),
            b' '       => (),
            b'1'..=b'9' => header = false,
            _         => println!("Unexpected character {} on header", chars[i])
        }
        i += 4;
    }
    if !header {
        print!("Initial stack top: ");
        print_stack_top(stacks);
    }
    header
}

fn parse_move(stacks: &mut [Vec<u8>], line: &str, part2: bool)
{
    let tokens = line.split_ascii_whitespace();
    let moves: Vec<usize> = tokens.filter_map(|t| t.parse::<usize>().ok()).collect();
    if moves.len() < 3 {
        return;
    }
    //moves : number, stack start, stack end (and index them from 0)
    let (n, s, e) = (moves[0], moves[1] - 1 , moves[2] - 1);
    if part2 {
        /* Part2: move in bulk */
        let mut v = stacks[s].split_off(stacks[s].len() - n);
        stacks[e].append(&mut v);
    } else {
        /* Part 1: Pop/push repeteadly */
        for _i in 0..n {
            let item = stacks[s].pop().expect("Unstacking empty stack !");
            stacks[e].push(item);
        }
    }
    /* Debug */
    //print!("After move {} f {} to {}: ", n, s, e);
    //print_stack_top(stacks);
}

fn print_stack_top(stacks: &[Vec<u8>])
{
    for s in stacks.iter() {
        print!("{}", *s.last().unwrap_or(&b'*') as char);
    }
    println!("");
}

pub fn day5(filename: &Path, part2: bool)
{
    let mut stacks: [Vec<u8>; 9] = Default::default();
    let mut header = true;
    let lines = file_to_lines(filename);
    for line in lines {
        if header {
            header = parse_initial_stacks(&mut stacks, line.as_str());
        } else {
            parse_move(&mut stacks, line.as_str(), part2);
        }
    }
    print!("Final stack top: ");
    print_stack_top(&stacks);
}
