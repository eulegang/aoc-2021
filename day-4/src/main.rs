use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn main() {
    let game = utils::input::<Game>();
    println!("winning score: {}", game.run());
    println!("let the squidy win: {}", game.last_run());
}

#[derive(PartialEq, Debug, Clone)]
pub struct Game {
    seq: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    numbers: [u8; 25],
}

#[derive(PartialEq, Debug, Clone)]
pub struct Mark<'a> {
    board: &'a Board,
    marked: [bool; 25],
}

impl Game {
    fn run(&self) -> u32 {
        let mut marks = self.boards.iter().map(|b| Mark::new(b)).collect::<Vec<_>>();

        for s in self.seq.iter() {
            for m in &mut marks {
                m.mark(*s);
            }

            for m in &marks {
                if m.done() {
                    return *s as u32 * m.unmarked_sum();
                }
            }
        }

        unreachable!()
    }

    fn last_run(&self) -> u32 {
        let mut marks = self.boards.iter().map(|b| Mark::new(b)).collect::<Vec<_>>();
        let mut seqs = self.seq.iter();

        while let Some(s) = seqs.next() {
            for m in &mut marks {
                m.mark(*s);
            }

            if marks.len() == 1 && marks[0].done() {
                return *s as u32 * marks.remove(0).unmarked_sum();
            }

            marks.retain(|m| !m.done());
        }

        unreachable!()
    }
}

impl<'a> Mark<'a> {
    fn new(board: &'a Board) -> Mark {
        Mark {
            board,
            marked: [false; 25]
        }
    }

    fn mark(&mut self, num: u8) {
        for i in 0..25 {
            if *&self.board.numbers[i] == num {
                self.marked[i] = true;
                return;
            }
        }
    }

    fn done(&self) -> bool {
        for row in 0..5 {
            let mut works = true;

            for col in 0..5 {
                if !self.marked[row * 5 + col] {
                    works = false;
                }
            }

            if works {
                return true;
            }
        }

        for col in 0..5 {
            let mut works = true;

            for row in 0..5 {
                if !self.marked[row * 5 + col] {
                    works = false;
                }
            }

            if works {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut res = 0;

        for i in 0..25 {
            if !self.marked[i] {
                res += self.board.numbers[i] as u32;
            }
        }

        res
    }
}

impl<'a> std::fmt::Display for Mark<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..5 {
            for c in 0..4 {
                if self.marked[r * 5 + c] {
                    write!(fmt, "\x1b[31m{:>2}\x1b[0m ", self.board.numbers[r * 5 + c])?;
                } else {
                    write!(fmt, "{:>2} ", self.board.numbers[r * 5 + c])?;
                }
            }

            let c = 4;
            if self.marked[r * 5 + c] {
                write!(fmt, "\x1b[31m{:>2}\x1b[0m\n", self.board.numbers[r * 5 + c])?;
            } else {
                write!(fmt, "{:>2}\n", self.board.numbers[r * 5 + c])?;
            }
        }

        Ok(())
    }
}

fn num(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |num: &str| num.parse::<u8>())(input)
}

impl utils::Parsable for Game {
    fn parse(input: &str) -> IResult<&str, Game> {
        let (input, seq) = separated_list1(tag(","), num)(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, boards) = separated_list1(tag("\n\n"), Board::parse)(input)?;

        Ok((input, Game { seq, boards }))
    }
}

impl utils::Parsable for Board {
    fn parse(input: &str) -> IResult<&str, Board> {
        let (input, row1) = tuple((
            preceded(multispace0, terminated(num, multispace1)),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
        ))(input)?;

        let (input, row2) = tuple((
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
        ))(input)?;

        let (input, row3) = tuple((
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
        ))(input)?;

        let (input, row4) = tuple((
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
        ))(input)?;

        let (input, row5) = tuple((
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            terminated(num, multispace1),
            num,
        ))(input)?;

        Ok((
            input,
            Board {
                numbers: [
                    row1.0, row1.1, row1.2, row1.3, row1.4, row2.0, row2.1, row2.2, row2.3, row2.4,
                    row3.0, row3.1, row3.2, row3.3, row3.4, row4.0, row4.1, row4.2, row4.3, row4.4,
                    row5.0, row5.1, row5.2, row5.3, row5.4,
                ],
            },
        ))
    }
}

#[test]
fn test_input() {
    let game = utils::test_input::<Game>();

    assert_eq!(
        game,
        Game {
            seq: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            boards: vec![
                Board {
                    numbers: [
                        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1,
                        12, 20, 15, 19,
                    ],
                },
                Board {
                    numbers: [
                        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14,
                        21, 16, 12, 6,
                    ],
                },
                Board {
                    numbers: [
                        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5,
                        2, 0, 12, 3, 7,
                    ],
                }
            ]
        }
    );
}

#[test]
fn test_output() {
    let game = utils::test_input::<Game>();

    assert_eq!(game.run(), 4512);
}

#[test]
fn test_output_last() {
    let game = utils::test_input::<Game>();
    assert_eq!(game.last_run(), 1924);
}

