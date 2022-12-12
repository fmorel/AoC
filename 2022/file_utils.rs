use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub fn file_to_lines(filename: &Path) -> impl Iterator<Item = String>
{
    let file = File::open(filename)
                .expect("Could not open file");
    let lines = BufReader::new(file).lines();
    lines.map(|l| l.unwrap())
}
