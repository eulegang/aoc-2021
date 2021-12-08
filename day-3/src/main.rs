use nom::{character::complete::one_of, IResult};

fn main() {
    let bins = utils::input_vec::<Bin>();
    println!("power comsumption: {}", comsumption(&bins));
    println!("life support: {}", life_support(&bins));
}

fn comsumption(bins: &[Bin]) -> u32 {
    let b = common_bin(bins, None);
    let inv = b.clone().inv();

    let gamma: u32 = b.into();
    let epsilon: u32 = inv.into();

    gamma * epsilon
}

fn oxy(bins: &[Bin]) -> u32 {
    let mut bins = bins.to_vec();
    let mut removes = Vec::with_capacity(bins.len());
    let mut i = 0;

    loop {
        if bins.len() == 1 {
            break bins.remove(0).into();
        }
        let bin = common_bin(&bins, Some(Bit::On));
        let bit = &bin.bits[i];

        for (pos, b) in bins.iter().enumerate().rev() {
            if &b.bits[i] != bit {
                removes.push(pos);
            }
        }

        for r in removes.iter() {
            bins.remove(*r);
        }

        removes.clear();

        i += 1;
    }
}

fn co2(bins: &[Bin]) -> u32 {
    let mut bins = bins.to_vec();
    let mut removes = Vec::with_capacity(bins.len());
    let mut i = 0;

    loop {
        if bins.len() == 1 {
            break bins.remove(0).into();
        }
        let bin = common_bin(&bins, Some(Bit::On)).inv();
        let bit = &bin.bits[i];

        for (pos, b) in bins.iter().enumerate().rev() {
            if &b.bits[i] != bit {
                removes.push(pos);
            }
        }

        for r in removes.iter() {
            bins.remove(*r);
        }

        removes.clear();

        i += 1;
    }
}

fn life_support(bins: &[Bin]) -> u32 {
    oxy(bins) * co2(bins)
}

#[derive(Debug, PartialEq, Clone)]
enum Bit {
    On,
    Off
}

impl Default for Bit {
    fn default() -> Bit {
        Bit::Off
    }
}

impl std::ops::Not for Bit {
    type Output = Bit;
    fn not(self) -> Bit {
        match self {
            Bit::On => Bit::Off,
            Bit::Off => Bit::On,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Bin {
    bits: Vec<Bit>,
}

fn common_bin(bins: &[Bin], default: Option<Bit>) -> Bin {
    let width = bins[0].bits.len();
    let mut one_counts = vec![0; width];

    for bin in bins {
        for i in 0..width {
            if bin.bits[i] == Bit::On {
                one_counts[i] += 1;
            }
        }
    }

    let mut bits = Vec::new();

    for count in one_counts {
        if 2*count > bins.len() {
            bits.push(Bit::On);
        } else if 2 * count == bins.len() {
            bits.push(default.clone().unwrap_or_default());
        } else {
            bits.push(Bit::Off);
        }
    }

    Bin { bits }
}

impl utils::Parsable for Bin {
    fn parse(mut input: &str) -> IResult<&str, Bin> {
        let mut bits = Vec::new();
        while !input.is_empty() {
            let (n, b) = one_of("10")(input)?;

            bits.push(match b {
                '0' => Bit::Off,
                '1' => Bit::On,
                _ => unreachable!(),
            });

            input = n;
        }

        Ok((input, Bin { bits }))
    }
}

impl Bin {
    fn inv(mut self) -> Bin {
        for i in 0..self.bits.len() {
            self.bits[i] = match self.bits[i] {
                Bit::On => Bit::Off,
                Bit::Off => Bit::On,
            };
        }

        Bin { bits: self.bits }
    }
}

impl Into<u32> for Bin {
    fn into(self) -> u32 {
        let mut res = 0;
        for bit in self.bits {
            res <<= 1;

            if bit == Bit::On {
                res += 1;
            }
        }

        res
    }
}


#[test]
fn test_model() {
    let expected = vec![
        Bin { bits: vec![Bit::Off, Bit::Off, Bit::On, Bit::Off, Bit::Off] },
        Bin { bits: vec![Bit::On, Bit::On, Bit::On, Bit::On, Bit::Off] },
        Bin { bits: vec![Bit::On, Bit::Off, Bit::On, Bit::On, Bit::Off] },
        Bin { bits: vec![Bit::On, Bit::Off, Bit::On, Bit::On, Bit::On] },

        Bin { bits: vec![Bit::On, Bit::Off, Bit::On, Bit::Off, Bit::On] },
        Bin { bits: vec![Bit::Off, Bit::On, Bit::On, Bit::On, Bit::On] },
        Bin { bits: vec![Bit::Off, Bit::Off, Bit::On, Bit::On, Bit::On] },
        Bin { bits: vec![Bit::On, Bit::On, Bit::On, Bit::Off, Bit::Off] },

        Bin { bits: vec![Bit::On, Bit::Off, Bit::Off, Bit::Off, Bit::Off] },
        Bin { bits: vec![Bit::On, Bit::On, Bit::Off, Bit::Off, Bit::On] },
        Bin { bits: vec![Bit::Off, Bit::Off, Bit::Off, Bit::On, Bit::Off] },
        Bin { bits: vec![Bit::Off, Bit::On, Bit::Off, Bit::On, Bit::Off] },
    ];

    let actual = utils::test_input_vec::<Bin>();

    for i in 0..expected.len() {
        assert_eq!(expected[i], actual[i], "index: {}", i);
    }
}

#[test]
fn bin_2_u32() {
    let bin = Bin { bits: vec![Bit::On, Bit::Off, Bit::Off, Bit::On, Bit::Off] };
    let num: u32 = bin.into();

    assert_eq!(num, 18_u32);
}

#[test]
fn test_output() {
    let bin = utils::test_input_vec::<Bin>();

    assert_eq!(comsumption(&bin), 198);
}

#[test]
fn test_output2() {
    let bin = utils::test_input_vec::<Bin>();

    assert_eq!(life_support(&bin), 230);
}

