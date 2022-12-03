use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
struct Elf
{
    id: u32,
    calories: u32,
}

fn day_1(filename: &Path)
{
    let file = File::open(filename)
                .expect("Could not open file");
    let lines = BufReader::new(file).lines();
    let mut elves = Vec::new();
    let mut elf = Elf{id: 0, calories: 0};
    for line in lines {
        match line.unwrap().as_str() {
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

fn main()
{
    day_1(Path::new("inputs/day1.txt"));
}
