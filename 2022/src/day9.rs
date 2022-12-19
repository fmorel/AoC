#![allow(dead_code)]
use crate::file_utils::file_to_lines;
use std::path::Path;
use vector2d::Vector2D;
use std::collections::HashSet;

#[derive(Debug)]
struct Rope
{
    knots: [Vector2D<i32>; 10],
    tail_visited: HashSet<(i32,i32)>
}
impl Rope
{
    fn new() -> Rope
    {
        Rope {
            knots: [Vector2D::new(0, 0); 10],
            tail_visited: HashSet::new()
        }
    }

    fn update_tail(&mut self)
    {
        for i in 1..10 {
            self.update_knot(i as usize);
        }

        /* Add visited position to the set */
        self.tail_visited.insert(self.knots[9].into());
       // println!("Tail is in {:?}", self.knots[9]);
    }

    fn update_knot(&mut self, knot: usize)
    {
        let d = self.knots[knot-1] - self.knots[knot];
        let k = &mut self.knots[knot];
        let (x, y): (i32, i32) = d.into();
        /* Movement table is actually simple :
         * Amplitude should be within the 2-square
         * In the 1-square, head is stil touching tail: no update
         * In the (+-2, +-1) or (+-1, +-2) position it is a diagonal move
         * Othorgonal move for the (+-2, 0) or (0, +-2) positions
         * Implement corner for part 2 since knot can move diagonnaly !!! */
        assert!(x.abs() <= 2 && y.abs() <= 2);
        match (x,y) {
            ( 0, 2) => k.y += 1,
            ( 2, 0) => k.x += 1,
            ( 0,-2) => k.y -= 1,
            (-2, 0) => k.x -= 1,
            ( 1, 2) | ( 2, 1) | ( 2, 2) => *k += Vector2D::new( 1, 1),
            ( 1,-2) | ( 2,-1) | ( 2,-2) => *k += Vector2D::new( 1,-1),
            (-1,-2) | (-2,-1) | (-2,-2) => *k += Vector2D::new(-1,-1),
            (-1, 2) | (-2, 1) | (-2, 2) => *k += Vector2D::new(-1, 1),
            _ => ()
        }
    }

    fn move_head(&mut self, dir: u8, steps: u8)
    {
        let mut m = Vector2D::new(0, 0);
        match dir {
            b'U' => m.y = 1,
            b'R' => m.x = 1,
            b'D' => m.y =-1,
            b'L' => m.x =-1,
            c => panic!("Unexpected move {}", c)
        }
        for _i in 0..steps {
            self.knots[0] += m;
            self.update_tail();
        }
    }
}

pub fn day9(filename: &Path)
{
    let lines = file_to_lines(filename);
    let mut rope = Rope::new();
    /* Insrt starting position */
    rope.tail_visited.insert((0,0));
    for l in lines {
        let tokens: Vec<&str> = l.split_ascii_whitespace().collect();
        assert!(tokens[0].len() == 1);
        assert!(tokens[1].len() <= 2);
        rope.move_head(tokens[0].as_bytes()[0], tokens[1].parse::<u8>().unwrap());
    }
    println!("Rope visited {} points", rope.tail_visited.len());
}
