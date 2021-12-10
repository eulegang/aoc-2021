use nom::{
    IResult,

    bytes::complete::tag,
    multi::separated_list1,
};


fn main() {
    let school = utils::input::<School>();
    println!("fish after 80 days: {}", school.sim_day(80));
    println!("fish after 256 days: {}", school.fast_sim_day(256));
}

#[derive(Debug, PartialEq, Clone)]
pub struct School {
    fish: Vec<Fish>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fish {
    left: u8,
}

impl School {
    fn sim_day(&self, mut days: u32) -> u64 {
        let mut fish = self.fish.clone();

        while days > 0 {
            let mut adds = 0;
            for f in &fish {
                if f.reproduce() {
                    adds += 1;
                }
            }


            for f in &mut fish {
                f.dec();
            }

            for _ in 0..adds {
                fish.push(Fish::default());
            }

            days -= 1;
        }
        
        fish.len() as u64
    }

    fn fast_sim_day(&self, mut days: u32) -> u64 {
        let mut lifecycle = [0u64; 9];

        for f in &self.fish {
            lifecycle[f.left as usize] += 1;
        }

        while days > 0 {
            let repro = lifecycle[0];
            for i in 1..lifecycle.len() {
                lifecycle[i-1] = lifecycle[i];
            }

            lifecycle[8] = repro;
            lifecycle[6] += repro;

            days -= 1;
        }

        let mut res = 0;
        for section in &lifecycle {
            res += *section;
        }

        res
    }
}

impl Fish {
    fn dec(&mut self) {
        self.left = self.left.checked_sub(1).unwrap_or(6);
    }

    fn reproduce(&self) -> bool {
        self.left == 0
    }
}

impl Default for Fish {
    fn default() -> Fish {
        Fish { left: 8 }
    }
}


impl utils::Parsable for School {
    fn parse(input: &str) -> IResult<&str, School> {
        let (input, fish) = separated_list1(tag(","), Fish::parse)(input)?;

        Ok((input, School { fish }))
    }
}

impl utils::Parsable for Fish {
    fn parse(input: &str) -> IResult<&str, Fish> {
        let (input, left) = nom::character::complete::u8(input)?;

        Ok((input, Fish { left }))
    }
}

#[test]
fn test_input() {
    let s = utils::test_input::<School>();

    assert_eq!(
        s,
        School {
            fish: vec![
                Fish { left: 3 },
                Fish { left: 4 },
                Fish { left: 3 },
                Fish { left: 1 },
                Fish { left: 2 },
            ]
        }
    )
}

#[test]
fn test_part1() {
    let s = utils::test_input::<School>();

    assert_eq!(s.sim_day(18), 26);
    assert_eq!(s.sim_day(80), 5934);
}

#[test]
fn test_part2() {
    let s = utils::test_input::<School>();

    assert_eq!(s.fast_sim_day(18), 26);
    assert_eq!(s.fast_sim_day(80), 5934);
    assert_eq!(s.fast_sim_day(256), 26984457539);
}
