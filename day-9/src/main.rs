use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

use std::collections::{HashSet, VecDeque};

fn main() {
    let grid: Grid = utils::input();

    println!("Risk Level: {}", grid.risk_level());
    println!("Basin Level: {}", grid.basin_level());
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    tiles: Vec<Vec<u8>>,
}

impl Grid {
    fn risk_level(&self) -> u32 {
        let mut risk = 0;

        for (x, y) in self.low_points() {
            risk += self.tiles[x][y] as u32 + 1;
        }

        risk
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let max_x = self.tiles.len();
        let max_y = self.tiles[0].len();
        let mut res = Vec::new();

        for x in 0..max_x {
            for y in 0..max_y {
                let value = self.tiles[x][y];
                let mut lowest = true;

                for (x, y) in QuadIter::new(x, y, max_x, max_y) {
                    lowest &= value < self.tiles[x][y];
                }

                if lowest {
                    res.push((x, y))
                }
            }
        }

        res
    }

    // really the product of the three largest basins
    fn basin_level(&self) -> u32 {
        let mut v = Vec::new();

        for p in self.low_points() {
            v.push(self.basin_area(p));
        }

        v.sort();
        v.reverse();

        v[0..3].iter().product()
    }

    fn basin_area(&self, point: (usize, usize)) -> u32 {
        let mut basin = HashSet::new();
        let mut queue = VecDeque::new();

        basin.insert(point);
        queue.push_front(point);

        while let Some(p) = queue.pop_back() {
            for p in QuadIter::new(p.0, p.1, self.tiles.len(), self.tiles[0].len()) {
                if !basin.contains(&p) && self.tiles[p.0][p.1] != 9 {
                    basin.insert(p);
                    queue.push_front(p);
                }
            }
        }

        basin.len() as u32
    }
}

pub struct QuadIter {
    x: usize,
    y: usize,

    max_x: usize,
    max_y: usize,
    stage: u8,
}

impl QuadIter {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> QuadIter {
        let stage = 0;

        QuadIter {
            x,
            y,
            max_x,
            max_y,
            stage,
        }
    }
}

impl Iterator for QuadIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        loop {
            let stage = self.stage;
            self.stage += 1;
            match stage {
                0 if self.y + 1 < self.max_y => return Some((self.x, self.y + 1)),
                1 if self.x + 1 < self.max_x => return Some((self.x + 1, self.y)),
                2 if self.x != 0 => return Some((self.x - 1, self.y)),
                3 if self.y != 0 => return Some((self.x, self.y - 1)),

                0 | 1 | 2 | 3 => (),

                _ => return None,
            };
        }
    }
}

impl utils::Parsable for Grid {
    fn parse(input: &str) -> IResult<&str, Grid> {
        fn tile(input: &str) -> IResult<&str, u8> {
            let (input, num) = one_of("0123456789")(input)?;

            Ok((input, num as u8 - b'0'))
        }
        let (input, tiles) = many1(terminated(many1(tile), opt(newline)))(input)?;
        Ok((input, Grid { tiles }))
    }
}

#[test]
fn test_input() {
    let grid: Grid = utils::test_input();
    assert_eq!(
        grid,
        Grid {
            tiles: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1,],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2,],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9,],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8,]
            ]
        }
    );
}

#[test]
fn test_risk_level() {
    let grid: Grid = utils::test_input();

    assert_eq!(grid.risk_level(), 15);
}

#[test]
fn test_basin_level() {
    let grid: Grid = utils::test_input();

    assert_eq!(grid.basin_level(), 1134);
}
