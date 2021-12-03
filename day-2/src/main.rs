use nom::{bytes::complete::tag, branch::alt, combinator::map_res, character::complete::digit1, IResult};

fn main() {
    let input = utils::input_vec::<Motion>();


    let (depth, hor) = resolve_position(&input);

    println!("calculation: {}", depth * hor);

    let (depth, hor) = resolve_aim_position(&input);

    println!("part2 calculation: {}", depth * hor);
}

fn resolve_aim_position(motions: &[Motion]) -> (u32, u32) {
    let mut depth = 0;
    let mut hor = 0;
    let mut aim: i32 = 0;

    for m in motions {
        match m {
            Motion::Forward(m) => {
                hor += m;
                depth = (depth as i32 + (aim * *m as i32)) as u32
            }
            Motion::Up(m) => aim -= *m as i32,
            Motion::Down(m) => aim += *m as i32,
        };
    }

    (depth, hor)
}

fn resolve_position(motions: &[Motion]) -> (u32, u32) {
    let mut depth = 0;
    let mut hor = 0;

    for m in motions {
        match m {
            Motion::Forward(m) => hor += m,
            Motion::Up(m) => depth -= m,
            Motion::Down(m) => depth += m,
        };
    }

    (depth, hor)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Motion {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl utils::Parsable for Motion {
    fn parse(input: &str) -> IResult<&str, Motion> {
        let (input, dir) = alt((tag("forward"), tag("down"), tag("up")))(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, mag) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;

        Ok((input, match dir {
            "forward" => Motion::Forward(mag),
            "down" => Motion::Down(mag),
            "up" => Motion::Up(mag),
            _ => unreachable!(),
        }))
    }
}

#[test]
fn test_model() {
    assert_eq!(utils::test_input_vec::<Motion>(), vec![
        Motion::Forward(5),
        Motion::Down(5),
        Motion::Forward(8),
        Motion::Up(3),
        Motion::Down(8),
        Motion::Forward(2),
    ]);
}

#[test]
fn test_result() {
    let (depth, hor) = resolve_position(&utils::test_input_vec::<Motion>());

    assert_eq!(depth * hor, 150);
}

#[test]
fn test_2result() {
    let (depth, hor) = resolve_aim_position(&utils::test_input_vec::<Motion>());

    assert_eq!(depth * hor, 900);
}
