use nom::{
    character::complete::one_of,
    bytes::complete::tag,
    multi::many1,
    IResult
};

#[cfg(test)]
use pretty_assertions::assert_eq;

fn main() {
    let mut clocks = utils::input_vec::<Record>();
    for clock in &mut clocks {
        clock.infer_digits();
    }

    let mut x = 0;
    let mut total = 0;

    for clock in &clocks {
        x += clock.uniq_digit_count();
        total += clock.output_value();
    }
    println!("uniq digit count: {}", x);
    println!("total: {}", total);
}

#[derive(Clone, PartialEq, Copy)]
pub struct Digit(u8);

impl std::fmt::Debug for Digit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Digit(\"")?;

        if *self & Digit::A == Digit::A { write!(fmt, "a")?; }
        if *self & Digit::B == Digit::B { write!(fmt, "b")?; }
        if *self & Digit::C == Digit::C { write!(fmt, "c")?; }
        if *self & Digit::D == Digit::D { write!(fmt, "d")?; }
        if *self & Digit::E == Digit::E { write!(fmt, "e")?; }
        if *self & Digit::F == Digit::F { write!(fmt, "f")?; }
        if *self & Digit::G == Digit::G { write!(fmt, "g")?; }

        write!(fmt, "\")")?;

        Ok(())
    }
}

impl Digit {
    const A: Digit = Digit(1);
    const B: Digit = Digit(2);
    const C: Digit = Digit(4);
    const D: Digit = Digit(8);
    const E: Digit = Digit(16);
    const F: Digit = Digit(32);
    const G: Digit = Digit(64);
}

impl std::ops::BitOr for Digit {
    type Output = Digit;

    fn bitor(self, rhs: Digit) -> Digit {
        Digit(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Digit {
    type Output = Digit;

    fn bitand(self, rhs: Digit) -> Digit {
        Digit(self.0 & rhs.0)
    }
}

impl std::ops::Not for Digit {
    type Output = Digit;
    fn not(self) -> Digit {
        Digit(!self.0)
    }
}

impl Default for Digit {
    fn default() -> Digit {
        Digit(0)
    }
}

impl Digit {
    fn on(&self) -> u8 {
        self.0.count_ones() as u8
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    digits: [Digit; 10],
    output: [Digit; 4]
}

impl Record {
    fn uniq_digit_count(&self) -> u32 {
        let mut c = 0;
        for out in &self.output {
            c += if matches!(out.on(), 2 | 3 | 4 | 7) { 1 } else { 0 }
        }

        c
    }

    fn infer_digits(&mut self) {
        let mut digits: [Digit; 10] = [
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
            Digit::default(),  
        ];

        for dig in &self.digits {
            match dig.on() {
                2 => digits[1] = *dig,
                3 => digits[7] = *dig,
                4 => digits[4] = *dig,
                7 => digits[8] = *dig,
                _ => (),
            };
        }

        for dig in &self.digits {
            if dig.on() == 6 {
                if (*dig & digits[1]) != digits[1] {
                    digits[6] = *dig;
                } else if (*dig & digits[4]) == digits[4] {
                    digits[9] = *dig;
                } else {
                    digits[0] = *dig;
                }
            }
        }

        for dig in &self.digits {
            if dig.on() == 5 {
                if (*dig & digits[1]) == digits[1] {
                    digits[3] = *dig;
                } else if (*dig & digits[9]) == *dig {
                    digits[5] = *dig;
                } else {
                    digits[2] = *dig;
                }
            }
        }


        self.digits.copy_from_slice(&digits);
    }

    fn output_value(&self) -> u32 {
        let mut out = 0;
        for digit in &self.output {
            for i in 0..10 {
                if self.digits[i] == *digit  {
                    out *= 10;
                    out += i as u32;

                    break;
                }
            }

        }

        out
    }
}

impl utils::Parsable for Record {
    fn parse(input: &str) -> IResult<&str, Record> {
        let (input, a) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, b) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, c) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, d) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, e) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, f) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, g) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, h) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, j) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, k) = Digit::parse(input)?;

        let (input, _) = tag(" | ")(input)?;

        let (input, one) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, two) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, three) = Digit::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, four) = Digit::parse(input)?;

        Ok((input, Record {
            digits: [a, b, c, d, e, f, g, h, j, k],
            output: [one, two, three, four],
        }))
    }
}

impl utils::Parsable for Digit {
    fn parse(input: &str) -> nom::IResult<&str, Digit> {
        let mut digit = Default::default();
        let (input, chs) = many1(one_of("abcdefg"))(input)?;

        for ch in chs {
            digit = digit | match ch {
                'a' => Digit::A,
                'b' => Digit::B,
                'c' => Digit::C,
                'd' => Digit::D,
                'e' => Digit::E,
                'f' => Digit::F,
                'g' => Digit::G,
                _ => unreachable!(),
            };
        }

        Ok((input, digit))
    }
}

#[test]
fn test_input() {
    let clocks = utils::test_input_vec::<Record>();

    assert_eq!(clocks[0], Record {
        digits: [
            Digit::B | Digit::E,
            Digit::C | Digit::F | Digit::B | Digit::E | Digit::G | Digit::A | Digit::D,
            Digit::C | Digit::B | Digit::D | Digit::G | Digit::E | Digit::F,
            Digit::F | Digit::G | Digit::A | Digit::E | Digit::C | Digit::D,
            Digit::C | Digit::G | Digit::E | Digit::B,
            Digit::F | Digit::D | Digit::C | Digit::G | Digit::E,
            Digit::A | Digit::G | Digit::E | Digit::B | Digit::F | Digit::D,
            Digit::F | Digit::E | Digit::C | Digit::D | Digit::B,
            Digit::F | Digit::A | Digit::B | Digit::C | Digit::D,
            Digit::E | Digit::D | Digit::B,
        ],
        output: [
            Digit::F | Digit::D | Digit::G | Digit::A | Digit::C | Digit::B | Digit::E,
            Digit::C | Digit::E | Digit::F | Digit::D | Digit::B,
            Digit::C | Digit::E | Digit::F | Digit::B | Digit::G | Digit::D,
            Digit::G | Digit::C | Digit::B | Digit::E,
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
fn infer_digits() {
    let mut record = Record {
        digits: [
            Digit::B | Digit::E,
            Digit::C | Digit::F | Digit::B | Digit::E | Digit::G | Digit::A | Digit::D,
            Digit::C | Digit::B | Digit::D | Digit::G | Digit::E | Digit::F,
            Digit::F | Digit::G | Digit::A | Digit::E | Digit::C | Digit::D,
            Digit::C | Digit::G | Digit::E | Digit::B,
            Digit::F | Digit::D | Digit::C | Digit::G | Digit::E,
            Digit::A | Digit::G | Digit::E | Digit::B | Digit::F | Digit::D,
            Digit::F | Digit::E | Digit::C | Digit::D | Digit::B,
            Digit::F | Digit::A | Digit::B | Digit::C | Digit::D,
            Digit::E | Digit::D | Digit::B,
        ],
        output: [
            Digit::F | Digit::D | Digit::G | Digit::A | Digit::C | Digit::B | Digit::E,
            Digit::C | Digit::E | Digit::F | Digit::D | Digit::B,
            Digit::C | Digit::E | Digit::F | Digit::B | Digit::G | Digit::D,
            Digit::G | Digit::C | Digit::B | Digit::E,
        ],
    };

    record.infer_digits();

    // Unambiguous
    assert_eq!(record.digits[1], Digit::B | Digit::E, "Infering 1");
    assert_eq!(record.digits[7], Digit::B | Digit::E | Digit::D, "Infering 7");
    assert_eq!(record.digits[8], Digit::C | Digit::F | Digit::B | Digit::E | Digit::G | Digit::A |  Digit::D, "Infering 8");
    assert_eq!(record.digits[4], Digit::C | Digit::G | Digit::E | Digit::B, "Infering 4");

    // infer from 1 missing
    assert_eq!(record.digits[6], Digit::F | Digit::G | Digit::A | Digit::E | Digit::C | Digit::D, "Infering 6");
    assert_eq!(record.digits[9], Digit::C | Digit::B | Digit::D | Digit::G | Digit::E | Digit::F , "Infering 9");
    assert_eq!(record.digits[0], Digit::A | Digit::G | Digit::E | Digit::B | Digit::F | Digit::D , "Infering 0");

    // infer from 2 missing
    assert_eq!(record.digits[3], Digit::F | Digit::E | Digit::C | Digit::D | Digit::B , "Infering 3");
    assert_eq!(record.digits[5], Digit::F | Digit::D | Digit::C | Digit::G | Digit::E, "Infering 5");
    assert_eq!(record.digits[2], Digit::F | Digit::A | Digit::B | Digit::C | Digit::D, "Infering 2");
}

#[test]
fn test_output_part2() {
    let mut clocks = utils::test_input_vec::<Record>();

    let mut x = 0;

    for clock in &mut clocks {
        clock.infer_digits();
        x += clock.output_value();
    }

    assert_eq!(x, 61229);
}
