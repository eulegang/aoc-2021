use nom::{
    IResult,
    character::complete::u32 as dword,
    multi::separated_list1,
    bytes::complete::tag,
};

fn main() {
    println!("minimum fuel cost: {}", utils::input::<Crabs>().reposition_fuel(linear_cost));
    println!("minimum componded fuel cost: {}", utils::input::<Crabs>().reposition_fuel(componded_cost));
}

#[derive(Debug, PartialEq, Clone)]
struct Crabs {
    pos: Vec<u32>,
}

impl Crabs {
    fn reposition_fuel(&self, dist: impl Fn(u32, u32) -> u32 + Clone + Copy) -> u32 {
        let mut pos = self.pos.iter().sum::<u32>() / self.pos.len() as u32;

        let cur = self.fuel_for(pos, dist);
        let up = self.fuel_for(pos+1, dist);
        let down = self.fuel_for(pos-1, dist);

        if cur < up && cur < down {
            return cur;
        }

        if cur > down {
            pos -= 1;
            let mut cur = cur;
            let mut next = down;

            while cur > next {
                cur = self.fuel_for(pos, dist);
                next = self.fuel_for(pos - 1, dist);

                pos -= 1;
            }

            return cur;
        } else {
            let mut cur = cur;
            let mut next = up;

            while cur > next {
                cur = self.fuel_for(pos, dist);
                next = self.fuel_for(pos + 1, dist);

                pos += 1;
            }

            return cur;

        }
    }

    fn fuel_for(&self, pos: u32, dist: impl Fn(u32, u32) -> u32) -> u32 {
        let mut res = 0;

        for p in &self.pos {
            res += dist(pos, *p);
        }

        res
    }
}

fn linear_cost(a: u32, b: u32) -> u32 {
    diff(a, b)
}

fn sum(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn componded_cost(a: u32, b: u32) -> u32 {
    let min = a.min(b);
    let max = a.max(b);

    if min == max {
        return 0;
    }


    sum(max - min)
}

fn diff(a: u32, b: u32) -> u32 {
    b.checked_sub(a).unwrap_or_else(|| a - b)
}

impl utils::Parsable for Crabs {
    fn parse(input: &str) -> IResult<&str, Crabs> {
        let (input, pos) = separated_list1(tag(","), dword)(input)?;

        Ok((input, Crabs { pos }))
    }
}

#[test]
fn test_input() {
    let crabs = utils::test_input::<Crabs>();

    assert_eq!(crabs, Crabs {
        pos: vec![
            16,1,2,0,4,2,7,1,2,14
        ]
    })
}

#[test]
fn test_output_part1() {
    let crabs = utils::test_input::<Crabs>();

    assert_eq!(crabs.reposition_fuel(linear_cost), 37);
}

#[test]
fn test_output_part2() {
    let crabs = utils::test_input::<Crabs>();

    assert_eq!(crabs.reposition_fuel(componded_cost), 168);
}

#[test]
fn test_componded() {
    assert_eq!(componded_cost(16, 5), 66);
    assert_eq!(componded_cost(5, 16), 66);
    assert_eq!(componded_cost(5, 5), 0);
}
