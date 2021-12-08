use nom::{
    character::complete::digit1,
    bytes::complete::tag,
    combinator::map_res,
    IResult
};

fn main() {
    let lines = utils::input_vec::<Line>();
    let mut grid = Grid::<1000>::default();
    for line in lines {
        if line.snapped() {
            grid.imprint(&line);
        }
    }

    println!("hazard square count: {}", grid.count(|d| d>= 2));
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn snapped(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<const W: usize> {
    quant: [[u8; W]; W],
}

impl<const W: usize> Default for Grid<W> {
    fn default() -> Grid<W> {
        Grid { quant: [[0u8; W]; W] }
    }
}

impl<const W: usize> Grid<W> {
    fn imprint(&mut self, line: &Line) {
        if line.start.x == line.end.x {
            let lo = line.start.y.min(line.end.y) as usize;
            let hi = line.start.y.max(line.end.y) as usize;
            let x = line.start.x as usize;

            for y in lo..=hi {
                self.quant[x][y] += 1;
            }
        } else {
            let lo = line.start.x.min(line.end.x) as usize;
            let hi = line.start.x.max(line.end.x) as usize;
            let y = line.start.y as usize;

            for x in lo..=hi {
                self.quant[x][y] += 1;
            }
        }
    }

    fn count(&self, pred: impl Fn(u8) -> bool) -> u32 {
        let mut res = 0;
        for x in 0..W {
            for y in 0..W {
                if pred(self.quant[x][y]) {
                    res += 1;
                }
            }
        }

        res
    }
}

impl utils::Parsable for Line {
    fn parse(input: &str) -> IResult<&str, Line> {
        let (input, start) = Point::parse(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, end) = Point::parse(input)?;

        Ok((input, Line { start, end }))
    }
}

impl utils::Parsable for Point {
    fn parse(input: &str) -> IResult<&str, Point> {
        let (input, x) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;

        Ok((input, Point { x, y }))
    }
}

impl<const W: usize> std::fmt::Display for Grid<W> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..W {
            for x in 0..W {
                match self.quant[x][y] {
                    0 => write!(fmt, ".")?,
                    q => write!(fmt, "{}", q)?,
               }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

#[test]
fn test_input() {
    let lines = utils::test_input_vec::<Line>();
    assert_eq!(
        lines,
        vec![
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            },
            Line {
                start: Point { x: 8, y: 0 },
                end: Point { x: 0, y: 8 }
            },
            Line {
                start: Point { x: 9, y: 4 },
                end: Point { x: 3, y: 4 }
            },
            Line {
                start: Point { x: 2, y: 2 },
                end: Point { x: 2, y: 1 }
            },
            Line {
                start: Point { x: 7, y: 0 },
                end: Point { x: 7, y: 4 }
            },
            Line {
                start: Point { x: 6, y: 4 },
                end: Point { x: 2, y: 0 }
            },
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 2, y: 9 }
            },
            Line {
                start: Point { x: 3, y: 4 },
                end: Point { x: 1, y: 4 }
            },
            Line {
                start: Point { x: 0, y: 0 },
                end: Point { x: 8, y: 8 }
            },
            Line {
                start: Point { x: 5, y: 5 },
                end: Point { x: 8, y: 2 }
            },
        ]
    );
}

#[test]
fn test_depth() {
    let lines = utils::test_input_vec::<Line>();
    let mut grid = Grid::<10>::default();
    for line in lines {
        if line.snapped() {
            grid.imprint(&line);
            println!("applying: {:?}", line);
            println!("{}", &grid);
        }
    }

    println!("{}", &grid);

    assert_eq!(grid.count(|d| d >= 2), 5);
}
