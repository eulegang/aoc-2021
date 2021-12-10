use nom::{
    IResult,
    character::complete::u32 as dword,
    multi::separated_list1,
    bytes::complete::tag,
};

fn main() {
    println!("minimum fuel cost: {}", utils::input::<Crabs>().reposition_fuel());
}

#[derive(Debug, PartialEq, Clone)]
struct Crabs {
    pos: Vec<u32>,
}

impl Crabs {
    fn reposition_fuel(&self) -> u32 {
        let mut pos = self.pos.iter().sum::<u32>() / self.pos.len() as u32;

        let cur = self.fuel_for(pos);
        let up = self.fuel_for(pos+1);
        let down = self.fuel_for(pos-1);

        if cur < up && cur < down {
            return cur;
        }

        if cur > down {
            pos -= 1;
            let mut cur = cur;
            let mut next = down;

            while cur > next {
                cur = self.fuel_for(pos);
                next = self.fuel_for(pos - 1);

                pos -= 1;
            }

            return cur;
        } else {
            let mut cur = cur;
            let mut next = up;

            while cur < next {
                cur = self.fuel_for(pos);
                next = self.fuel_for(pos + 1);

                pos += 1;
            }

            return cur;

        }
    }

    fn fuel_for(&self, pos: u32) -> u32 {
        let mut res = 0;

        for p in &self.pos {
            res += diff(pos, *p);
        }

        res
    }
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

    assert_eq!(crabs.reposition_fuel(), 37);
}
