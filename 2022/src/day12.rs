use crate::file_utils::file_to_lines;
use std::path::Path;
use array2d::Array2D;
use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
struct Terrain
{
    height: u8,
    start: bool,
    end: bool,
    coord: (usize, usize),
    neighbours: Vec<(usize,usize)>,
    distance: Option<u16>
}

impl Terrain
{
    fn new(code: u8) -> Terrain
    {
        match code {
            b'S' =>  Terrain {
                        height: 0,
                        start: true,
                        distance: Some(0),
                        ..Default::default()
                    },
            b'E' =>  Terrain {
                        height: 25,
                        end: true,
                        ..Default::default()
                    },
            c   =>  Terrain {
                        height: c - b'a',
                        ..Default::default()
                    }
        }
    }

    fn set_coord(&mut self, (x,y): (usize,usize), (h,w): (usize,usize))
    {
        self.coord = (x, y);
        if x > 0 {
            self.neighbours.push((x-1,y));
        }
        if y > 0 {
            self.neighbours.push((x,y-1));
        }
        if x < h-1 {
            self.neighbours.push((x+1,y));
        }
        if y < w-1 {
            self.neighbours.push((x, y+1));
        }
    }

    fn can_reach(&self, neighbour: &Terrain) -> bool 
    {
        neighbour.height <= self.height+1
    }
}

pub fn day12(filename: &Path)
{
    /* Parse the grid */
    let lines = file_to_lines(filename);
    let mut rows: Vec<Vec<Terrain>> = Vec::new();
    for l in lines {
        let mut row: Vec<Terrain> = Vec::new();
        for b in l.bytes() {
            row.push(Terrain::new(b));
        }
        rows.push(row);
    }
    let mut grid = Array2D::from_rows(&rows).unwrap();
    let w = grid.num_columns();
    let h = grid.num_rows();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut min_distance = 0;

    /* Fill in coordinate data and put start element in queue */
    for i in 0..h {
        for j in 0..w {
            grid[(i,j)].set_coord((i,j),(h,w));
            if grid[(i,j)].start {
                queue.push_back((i, j));
            }
        }
    }
    /* Breadth first search to find minimum distance */
    'bfs: while !queue.is_empty() {
        let (x,y) = queue.pop_front().unwrap();
        let t = grid[(x,y)].clone();
        /* Add reachable neighbours to the list if they are not marked yet 
         * Mark them with distance + 1*/
        for (u,v) in t.neighbours.iter() {
            let mut tn = &mut grid[(*u,*v)];
            if tn.distance.is_none() && t.can_reach(tn) {
                tn.distance = Some(t.distance.unwrap() + 1);
                queue.push_back((*u,*v));
                if tn.end {
                    min_distance = tn.distance.unwrap();
                    break 'bfs;
                }
            }
        }
    }
    println!("Minimum distance: {}", min_distance); 
}
