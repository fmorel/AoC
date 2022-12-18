#![allow(dead_code)]

use crate::file_utils::file_to_lines;
use std::path::Path;

/* Day 1 */
#[derive(Copy, Clone)]
struct Elf
{
    id: u32,
    calories: u32,
}

pub fn day_1(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut elves = Vec::new();
    let mut elf = Elf{id: 0, calories: 0};
    for line in lines {
        match line.as_str() {
            "" => {
                    elves.push(elf);
                    elf.calories = 0;
                    elf.id += 1;
                  },
             s => elf.calories += s.parse::<u32>().unwrap(),
        }
    }
    /* Add last elf */
    if elf.calories > 0 {
        elves.push(elf);
    }

    /* Reduce to retrieve the elf with the max calories */
    let max_elf = elves.iter().reduce(|acc, item| {
                if acc.calories >= item.calories { acc } else { item }
          }).unwrap();
    println!("Elf {} has {} calories", max_elf.id, max_elf.calories);

    /* Part 2: just sort the vector to find the max 3 elves */
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let calories_3elves = elves[0].calories + elves[1].calories + elves[2].calories;
    println!("The 3 best elvs carry a total of {} calories", calories_3elves);
}

/* Day 2 */
fn rps_index(play :&str) -> i32
{
    match play {
        "A X" => 0,
        "A Y" => 1,
        "A Z" => 2,
        "B X" => 3,
        "B Y" => 4,
        "B Z" => 5,
        "C X" => 6,
        "C Y" => 7,
        "C Z" => 8,
        ""    => 9,
        _     => -1
    }
}

fn rps_score(play :&str, part2: bool) -> u32
{
    /* Add the 'blank line' at index 9 */
    let score_tab = [1+3, 2+6, 3+0, 1+0, 2+3, 3+6, 1+6, 2+0, 3+3, 0];
    let score_tab_2 = [3+0, 1+3, 2+6, 1+0, 2+3, 3+6, 2+0, 3+3, 1+6, 0];
    let index = rps_index(play);
    assert!(index >= 0);
    if part2 {
        score_tab_2[index as usize]
    } else {
        score_tab[index as usize]
    }
}

pub fn day_2(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut points = 0;
    for line in lines {
        points += rps_score(line.as_str(), true);
    }
    println!("Total points is {}", points);
}

/* Day 3 */
/* Create a bitmap for each compartment where a bit are indexed by priority */
fn backpack_compartment_items(compartment: &str) -> u64
{
    let mut items_bmp: u64 = 0;
    // 'A' has ASCII 0x41 and 'a' 0x61
    for c in compartment.chars() {
        match c {
            'A'..='Z' => items_bmp |= 1 << ((c as u8) - 0x41u8 + 26),
            'a'..='z' => items_bmp |= 1 << ((c as u8) - 0x61u8),
            _ => println!("Unexpected character {} in backpack", c)
        }
    }
    items_bmp
}

fn backpack_get_priority(backpack: &str) -> u32
{
    //string length should be even
    assert!(backpack.len() % 2 == 0);
    //Split in half
    let (compt0, compt1) = backpack.split_at(backpack.len() / 2);
    let items_bmp0 = backpack_compartment_items(compt0);
    let items_bmp1 = backpack_compartment_items(compt1);
    let common_item = items_bmp0 & items_bmp1;
    //Return the index of the first common item (should be the first one, too !)
    common_item.trailing_zeros() + 1
}

pub fn day_3(filename: &Path, part2: bool)
{
    let lines = file_to_lines(filename);
    let mut priority = 0;
    let mut elf_idx = 0;
    let mut group_items_bmp: u64 = 0;
    for line in lines {
        if part2 {
            if elf_idx == 0 {
                group_items_bmp = backpack_compartment_items(line.as_str());
            } else {
                group_items_bmp &= backpack_compartment_items(line.as_str());
            }
            elf_idx += 1;
            if elf_idx == 3 {
                elf_idx = 0;
                priority += group_items_bmp.trailing_zeros() + 1;
            }
        } else {
            priority += backpack_get_priority(line.as_str());
        }
    }
    println!("Total priorities is {}", priority);
}

/* Day 4 */
/* Assignment is a bitmap , so it's easy to perfom union or intersection on them */
fn parse_assignment(range: &str) -> u128
{
    let (beg, end) = range.split_once('-').unwrap();
    let (beg_i, end_i) = (beg.parse::<u8>().unwrap(), end.parse::<u8>().unwrap());
    assert!(beg_i < 100 && end_i < 100);
    ((1 << (end_i+1)) - 1) & !((1 << beg_i) - 1)
}

fn parse_assignment_pair(pair: &str) -> (u128, u128)
{
    let (a1, a2) = pair.split_once(',').unwrap();
    (parse_assignment(a1), parse_assignment(a2))
}

pub fn day_4(filename: &Path, part2: bool)
{
    let lines = file_to_lines(filename);
    let mut overlaps = 0;
    for line in lines {
        let (a1, a2) = parse_assignment_pair(line.as_str());
        if part2 {
            if a1 & a2 != 0 {
                overlaps += 1
            }
        } else {
            let all: u128 = a1 | a2;
            if a1 == all || a2 == all {
                overlaps += 1
            }
        }
    }
    println!("There are {} overlaps", overlaps);
}
