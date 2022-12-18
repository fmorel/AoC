#![allow(dead_code)]
use crate::file_utils::file_to_lines;
use std::path::Path;
use array2d::Array2D;
use std::cmp;

/* Visibility data for a given direction */
#[derive(Default, Debug, Clone)]
struct Visibility
{
    max_height: i8, /* max height seen in this direction */
    visible: bool   /* visiblity status in this direction */
}

#[derive(Default, Debug, Clone)]
struct Tree
{
    height: i8,
    v_trbl: [Visibility; 4], /* visibility data: top, right, bottom, left */
}

impl Tree
{
    fn extend_visibility(&mut self, max_height: i8, dir: usize)
    {
        self.v_trbl[dir].max_height = max_height;
        self.v_trbl[dir].visible = self.height > self.v_trbl[dir].max_height;
    }

    fn neighbour_max_height(&self, dir: usize) -> i8
    {
        cmp::max(self.v_trbl[dir].max_height, self.height)
    }

    fn is_visible(&self) -> bool
    {
        self.v_trbl[0].visible || self.v_trbl[1].visible || self.v_trbl[2].visible || self.v_trbl[3].visible
    }
}


pub fn day8(filename: &Path)
{
    /* Parse the grid */
    let lines = file_to_lines(filename);
    let mut rows: Vec<Vec<Tree>> = Vec::new();
    for l in lines {
        let mut row: Vec<Tree> = Vec::new();
        for b in l.bytes() {
            let t = Tree {
                height: (b - b'0') as i8,
                ..Default::default()
            };
            row.push(t);
        }
        rows.push(row);
    }
    let mut grid = Array2D::from_rows(&rows).unwrap();
    let w = grid.num_columns();
    let h = grid.num_rows();
    let mut trees_visible = 0;
    /* Now we need to populate the visibility data.
     * We will need two passes :
     * - First pass, from top left to bottom right will propagate visiblity status for top and left direction
     * - Second pass, from bottom right to top left will update bottom and right direction.
     *   We will also use the second pass to perform the count of visible trees since all direction
     *   will be known */
    for i in 0..h {
        for j in 0..w {
            let mut max_height = -1;
            /* Edges behave like max_height == -1 in their direction */
            /* visibility top */
            if i > 0 {
                max_height = grid[(i-1, j)].neighbour_max_height(0);
            }
            grid[(i,j)].extend_visibility(max_height, 0);
            /* visibility left */
            max_height = -1;
            if j > 0 {
                max_height = grid[(i, j-1)].neighbour_max_height(3);
            }
            grid[(i,j)].extend_visibility(max_height, 3);
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let mut max_height = -1;
            /* visibility bottom */
            if i < w-1 {
                max_height = grid[(i+1, j)].neighbour_max_height(2);
            }
            grid[(i,j)].extend_visibility(max_height, 2);
            /* visibility right */
            max_height = -1;
            if j < h-1 {
                max_height = grid[(i, j+1)].neighbour_max_height(1);
            }
            grid[(i,j)].extend_visibility(max_height, 1);
            /* Total visibility */
            if grid[(i,j)].is_visible() {
                trees_visible += 1;
            }
        }
    }
    //println!("{:#?}", grid);
    println!("Total trees visible: {}", trees_visible);
}
