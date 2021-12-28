use nom::{character::complete::{one_of, newline}, IResult, combinator::opt, sequence::terminated};
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut grid: Grid = utils::input();

    let mut total = 0;
    
    for _ in 0..100 {
        total += grid.step();
    }

    let mut i = 100;

    while !grid.synced_flashed() {
        grid.step();
        i += 1;
    }

    println!("100 steps: {}", total);
    println!("sync flash: {}", i);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    power: [[u8; 10]; 10],
}

impl Grid {
    fn step(&mut self) -> u64 {

        for x in 0..10 {
            for y in 0..10 {
                self.power[x][y] += 1;
            }
        }

        let mut burst = HashSet::new();
        let mut queue = VecDeque::new();

        for x in 0..10 {
            for y in 0..10 {
                if self.power[x][y] > 9 {
                    burst.insert((x, y));
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for nx in utils::neigh(x, 10) {
                for ny in utils::neigh(y, 10) {
                    if nx == x && ny == y {
                        continue;
                    }

                    self.power[nx][ny] = 10.min(self.power[nx][ny] + 1);

                    if self.power[nx][ny] > 9 && !burst.contains(&(nx, ny)) {
                        burst.insert((nx, ny));
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        let mut bursts = 0;

        for x in 0..10 {
            for y in 0..10 {
                if self.power[x][y] > 9 {
                    bursts += 1;
                    self.power[x][y] = 0;
                }
            }
        }

        bursts
    }

    fn synced_flashed(&self) -> bool {
        let mut res = 0;
        for x in 0..10 {
            for y in 0..10 {
                res |= self.power[x][y];
            }
        }

        res == 0
    }
}

impl utils::Parsable for Grid {
    fn parse(mut input: &str) -> IResult<&str, Grid> {
        fn level(input: &str) -> IResult<&str, u8> {
            let (input, ch) = one_of("0123456789")(input)?;
            Ok((input, ch as u8 - b'0'))
        }

        let mut power = [[0u8; 10]; 10];

        for i in 0..100 {
            let (rest, l) = terminated(level, opt(newline))(input)?;

            power[i/10][i%10] = l;
            
            input = rest;
        }

        Ok((input, Grid { power }))
    }
}

#[test]
fn input() {
    let grid: Grid = utils::test_input();

    assert_eq!(
        grid,
        Grid {
            power: [
                [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                [5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
            ]
        }
    )
}

#[test]
fn some_steps() {
    let mut grid: Grid = utils::test_input();

    assert_eq!(grid.step(), 0);
    grid.step();
    //assert_eq!(grid.step(), 35);

    assert_eq!(
        grid,
        Grid {
            power: [
                [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                [8, 7, 0, 0, 0, 0, 6, 8, 4, 8]
            ]
        }
    )
}

#[test]
fn part1() {
    let mut grid: Grid = utils::test_input();

    let mut total = 0;

    for _ in 0..100 {
        total += grid.step()
    }

    assert_eq!(total, 1656);
}


#[test]
fn part2() {
    let mut grid: Grid = utils::test_input();
    let mut i = 0;

    while !grid.synced_flashed() {
        grid.step();
        i += 1;
    }

    assert_eq!(i, 195);
}
