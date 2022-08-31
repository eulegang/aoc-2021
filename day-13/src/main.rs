use nom::bytes::complete::tag;
use nom::character::complete::{one_of, u32 as parse_u32};
use nom::multi::separated_list1;
use std::collections::HashSet;

fn main() {
    let page: Page = utils::input();

    println!("visible: {}", page.fold().visible());
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Page {
    points: Points,
    inst: Vec<Instruction>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Points(Vec<(u32, u32)>);

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Instruction {
    axis: Axis,
    line: u32,
}

impl Page {
    fn fold(self) -> Points {
        let mut points = self.points;
        for inst in self.inst {
            points = points.perform(inst).unique();
        }

        points
    }
}

impl Points {
    fn perform(self, inst: Instruction) -> Points {
        let mut points = self.0;

        for point in &mut points {
            match inst.axis {
                Axis::X => point.0 = fold(inst.line, point.0),
                Axis::Y => point.1 = fold(inst.line, point.1),
            }
        }

        Points(points)
    }

    fn unique(self) -> Points {
        let mut points = self.0;
        let mut uniq = HashSet::new();

        let mut i = 0;
        while i < points.len() {
            if uniq.contains(&points[i]) {
                points.swap_remove(i);
                continue;
            }

            uniq.insert(points[i]);

            i += 1;
        }

        Points(points)
    }

    fn visible(self) -> usize {
        self.0.len()
    }

    fn dimensions(&self) -> (u32, u32) {
        let mut x = 0;
        let mut y = 0;

        for p in &self.0 {
            x = x.max(p.0);
            y = y.max(p.1);
        }

        (x, y)
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (row, col) = self.dimensions();

        for r in 0..=row {
            for c in 0..=col {
                let mut found = false;
                for p in &self.0 {
                    if p.0 == c && p.1 == r {
                        found = true;
                    }
                }

                if found {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn fold(line: u32, level: u32) -> u32 {
    if line < level {
        line - (level - line)
    } else {
        level
    }
}

impl utils::Parsable for Page {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, p) = separated_list1(tag("\n"), point)(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, inst) = separated_list1(tag("\n"), Instruction::parse)(input)?;

        let points = Points(p);
        Ok((input, Page { points, inst }))
    }
}

impl utils::Parsable for Instruction {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, _) = tag("fold along ")(input)?;
        let (input, axis) = one_of("xy")(input)?;
        let axis = match axis {
            'x' => Axis::X,
            'y' => Axis::Y,
            _ => unreachable!(),
        };

        let (input, _) = tag("=")(input)?;
        let (input, line) = parse_u32(input)?;

        Ok((input, Instruction { axis, line }))
    }
}

fn point(input: &str) -> nom::IResult<&str, (u32, u32)> {
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_u32(input)?;

    Ok((input, (x, y)))
}

#[test]
fn valid_input() {
    let page: Page = utils::test_input();

    assert_eq!(
        page.points,
        Points(vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ])
    );

    assert_eq!(
        page.inst,
        vec![
            Instruction {
                axis: Axis::Y,
                line: 7,
            },
            Instruction {
                axis: Axis::X,
                line: 5,
            }
        ]
    );
}

#[test]
fn test_result_part1() {
    let page: Page = utils::test_input();
    println!("{}", page.points.clone());
    println!("{}", page.clone().points.perform(page.clone().inst[0]));
    println!("{}", page.clone().fold());

    assert_eq!(page.points.perform(page.inst[0]).visible(), 17);

    //assert_eq!(page.fold().visible(), 17);
}

#[test]
fn test_fold() {
    assert_eq!(8, fold(10, 12));
    assert_eq!(5, fold(10, 5));
}
