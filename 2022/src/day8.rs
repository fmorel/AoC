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
    visible: bool,   /* visiblity status in this direction */
    view_dist_by_height: [u8; 10],   /* viewing distance in this direction for any given tree height (part 2)*/
}

#[derive(Default, Debug, Clone)]
struct Tree
{
    height: i8,
    v_trbl: [Visibility; 4], /* visibility data: top, right, bottom, left */
}

impl Tree
{
    fn extend_visibility(&mut self, v: Visibility, dir: usize)
    {
        self.v_trbl[dir] = v;
        self.v_trbl[dir].visible = self.height > self.v_trbl[dir].max_height;
    }

    fn neighbour_visibility(&self, dir: usize) -> Visibility
    {
        let mut v : Visibility = self.v_trbl[dir].clone();
        v.max_height = cmp::max(v.max_height, self.height);
        /* viewing distance: reset to 1 for tree smaller or equal to current height
         * Increment for bigger heights */
        for i in 0..self.height+1 {
            v.view_dist_by_height[i as usize] = 1;
        }
        for i in (self.height+1)..10 {
            v.view_dist_by_height[i as usize] += 1;
        }
        v
    }

    fn is_visible(&self) -> bool
    {
        self.v_trbl[0].visible || self.v_trbl[1].visible || self.v_trbl[2].visible || self.v_trbl[3].visible
    }

    fn scenic_score(&self) -> u32
    {
        let mut s: u32 = 1;
        for i in 0..4 {
            s *= self.v_trbl[i].view_dist_by_height[self.height as usize] as u32;
        }
        s
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
    let mut scenic_score = 0;
    /* Now we need to populate the visibility data.
     * We will need three passes :
     * - First set the edges
     * - Then from top left to bottom right will propagate visiblity status for top and left direction
     * - Finally, from bottom right to top left will update bottom and right direction.
     *   We will also use the second pass to perform the count of visible trees since all direction
     *   will be known */
    /* Edges */
    for j in 0..w {
        grid[(0, j)].v_trbl[0].visible = true;
        grid[(h-1, j)].v_trbl[2].visible = true;
        trees_visible += 2;
    }
    for i in 0..h {
        grid[(i, 0)].v_trbl[3].visible = true;
        grid[(i, w-1)].v_trbl[1].visible = true;
        trees_visible += 2;
    }
    trees_visible -= 4;  /* corners were counted twice */

    /* Pass 1: top-left to bottom-right (without edges) */
    for i in 1..h-1 {
        for j in 1..w-1 {
            /* visibility top */
            let vt = grid[(i-1, j)].neighbour_visibility(0);
            grid[(i,j)].extend_visibility(vt, 0);
            /* visibility left */
            let vl = grid[(i, j-1)].neighbour_visibility(3);
            grid[(i,j)].extend_visibility(vl, 3);
        }
    }
    /* Pass 2: bottom-right to top left (without edges) */
    for i in (1..h-1).rev() {
        for j in (1..w-1).rev() {
            /* visibility bottom */
            let vb = grid[(i+1, j)].neighbour_visibility(2);
            grid[(i,j)].extend_visibility(vb, 2);
            /* visibility right */
            let vr = grid[(i, j+1)].neighbour_visibility(1);
            grid[(i,j)].extend_visibility(vr, 1);
            
            /* Total visibility */
            if grid[(i,j)].is_visible() {
                trees_visible += 1;
            }
            scenic_score = cmp::max(scenic_score, grid[(i,j)].scenic_score());
        }
    }
    //println!("{:#?} scenic {}", grid[(1, 2)], grid[(1, 2)].scenic_score());
    //println!("{:#?} scenic {}", grid[(3, 2)], grid[(3, 2)].scenic_score());
    println!("Total trees visible: {}", trees_visible);
    println!("Best scenic score: {}", scenic_score);
}
