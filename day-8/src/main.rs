use nom::{
    character::complete::one_of,
    bytes::complete::tag,
    multi::many1,
    IResult
};

fn main() {
    let clocks = utils::input_vec::<Record>();
    let mut x = 0;

    for clock in clocks {
        x += clock.uniq_digit_count();
    }
    println!("uniq digit count: {}", x);
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Digit {
    A, B, C, D, E, F, G
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    digits: Vec<Digit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    digits: [Number; 10],
    output: [Number; 4]
}

impl Number {
    fn uniq(&self) -> bool {
        let len = self.digits.len();

        matches!(len, 2 | 4 | 3 | 7)
    }
}

impl Record {
    fn uniq_digit_count(&self) -> u32 {
        let mut c = 0;
        for out in &self.output {
            c += if out.uniq() { 1 } else { 0 }
        }

        c
    }

    fn output_value(&self) -> u32 {


        todo!()
    }
}

impl utils::Parsable for Record {
    fn parse(input: &str) -> IResult<&str, Record> {
        let (input, a) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, b) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, c) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, d) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, e) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, f) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, g) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, h) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, j) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, k) = Number::parse(input)?;

        let (input, _) = tag(" | ")(input)?;

        let (input, one) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, two) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, three) = Number::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, four) = Number::parse(input)?;

        Ok((input, Record {
            digits: [a, b, c, d, e, f, g, h, j, k],
            output: [one, two, three, four],
        }))
    }
}

impl utils::Parsable for Number {
    fn parse(input: &str) -> IResult<&str, Number> {
        let (input, digits) = many1(Digit::parse)(input)?;

        Ok((input, Number { digits }))
    }
}

impl utils::Parsable for Digit {
    fn parse(input: &str) -> nom::IResult<&str, Digit> {
        let (input, ch) = one_of("abcdefg")(input)?;

        let digit = match ch {
            'a' => Digit::A,
            'b' => Digit::B,
            'c' => Digit::C,
            'd' => Digit::D,
            'e' => Digit::E,
            'f' => Digit::F,
            'g' => Digit::G,
            _ => unreachable!(),
        };

        Ok((input, digit))
    }
}

#[test]
fn test_input() {
    use Digit::*;
    let clocks = utils::test_input_vec::<Record>();

    assert_eq!(clocks[0], Record {
        digits: [
            Number { digits: vec![B, E], },
            Number { digits: vec![C, F, B, E, G, A, D], },
            Number { digits: vec![C, B, D, G, E, F], },
            Number { digits: vec![F, G, A, E, C, D], },
            Number { digits: vec![C, G, E, B], },
            Number { digits: vec![F, D, C, G, E], },
            Number { digits: vec![A, G, E, B, F, D], },
            Number { digits: vec![F, E, C, D, B], },
            Number { digits: vec![F, A, B, C, D], },
            Number { digits: vec![E, D, B], },
        ],
        output: [
            Number { digits: vec![F, D, G, A, C, B, E], },
            Number { digits: vec![C, E, F, D, B], },
            Number { digits: vec![C, E, F, B, G, D], },
            Number { digits: vec![G, C, B, E], },
        ],
    });
}

#[test]
fn test_output() {
    let clocks = utils::test_input_vec::<Record>();

    let mut x = 0;

    for clock in clocks {
        x += clock.uniq_digit_count();
    }

    assert_eq!(x, 26);
}

#[test]
#[ignore]
fn test_output_part2() {
    let clocks = utils::test_input_vec::<Record>();

    let mut x = 0;

    for clock in clocks {
        x += clock.output_value();
    }

    assert_eq!(x, 61229);
}
