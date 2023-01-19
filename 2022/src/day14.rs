use crate::file_utils::file_to_lines;
use std::path::Path;
use array2d::Array2D;

/* Grid is an u8
 * 0 = air
 * 1 = rock
 * 2 = sand
 */

fn parse_coord(coord: &str) -> (usize, usize)
{
    let (c0, c1) = coord.split_once(',').unwrap();
    (c0.parse::<usize>().unwrap() - 480, c1.parse::<usize>().unwrap())
}

fn parse_rock_path(grid: &mut Array2D<u8>, line: &str)
{
    let mut coords = line.split(" -> ");
    let c0 = coords.next().unwrap();
    let (mut x0, mut y0) = parse_coord(c0);
    for c in coords {
        let (x1, y1) = parse_coord(c);
        if x1 > x0 {
            for x in x0..x1+1 {
                grid.set(y0, x, 1).unwrap();
            }
        } else if x0 > x1 {
            for x in x1..x0+1 {
                grid.set(y0, x, 1).unwrap();
            }
        } else if y1 > y0 {
            for y in y0..y1+1 {
                grid.set(y, x0, 1).unwrap();
            }
        } else if y0 > y1 {
            for y in y1..y0+1 {
                grid.set(y, x0, 1).unwrap();
            }
        }
        (x0, y0) = (x1, y1);
    }
}

fn drop_grain(grid: &mut Array2D<u8>) -> bool
{
    let (mut x, mut y) = (20, 0);
    loop {
        let mut e = grid.get(y+1,x).unwrap();
        if *e == 0 {
            y = y+1;
            /* down in the abyss */
            if y > 195 {
                return false;
            }
            continue;
        }
        e = grid.get(y+1, x-1).unwrap();
        if *e == 0 {
            y = y+1;
            x = x-1;
            continue;
        }
        e = grid.get(y+1, x+1).unwrap();
        if *e == 0 {
            y = y+1;
            x = x+1;
            continue;
        }
        /* grain is settled */
        grid.set(y, x, 2).unwrap();
        break;
    }
    return true;
}

fn grid_print(grid: &Array2D<u8>)
{
    for row_iter in grid.rows_iter() {
        for e in row_iter {
            let c = match e {
                        0 => '.',
                        1 => '#',
                        2 => 'o',
                        _ => ' '
                    };
            print!("{}", c);
        }
        println!();
    }
}

pub fn day14(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut n_sand = 0;
    let mut grid : Array2D<u8> = Array2D::filled_with(0, 200, 120);
    for l in lines {
        parse_rock_path(&mut grid, &l);
    }
    grid_print(&grid);
    while drop_grain(&mut grid) {
        n_sand += 1;
    }
    println!("All grains settled after {}", n_sand);
    grid_print(&grid);
}
