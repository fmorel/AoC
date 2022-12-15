use file_utils::file_to_lines;
use std::path::Path;

pub fn day6(filename: &Path, part2: bool)
{
    /* Input is a single line, convert it to u8 iterator*/
    let line = file_to_lines(filename).next().unwrap();
    let chars = line.as_str().bytes();
    let mut window: Vec<u8> = Vec::new();
    let mut i = 0;
    let preamble_len = if part2 { 14 } else { 4 };
    for c in chars {
        if !window.is_empty() {
            /* If c is is in the window, reset it from index c+1 */
            match window.iter().position(|w| w == &c) {
                Some(i) => {window.drain(0..i+1); ()}
                None => ()
            }
        }
        window.push(c);
        i += 1;
        if window.len() == preamble_len {
            break;
        }
    }
    println!("Preamble found at index {}", i);
}
    
