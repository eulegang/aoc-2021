use nom::IResult;

use std::collections::VecDeque;

fn main() {
    let chunks = utils::input_vec();
    println!("Error Score: {}", error_score(&chunks));
    println!("Completion score: {}", mid_autocomplete_score(&chunks));
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    content: String,
}

fn error_score(chunks: &[Chunk]) -> u32 {
    let mut score = 0;
    for chunk in chunks {
        let s = match chunk.offending_char() {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        };

        score += s;
    }

    score
}

fn mid_autocomplete_score(chunks: &[Chunk]) -> u64 {
    let mut scores = Vec::new();

    for chunk in chunks {
        if let Some(s) = chunk.competion() {
            let mut score = 0;

            for ch in s.chars() {
                score *= 5;

                score += match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }

            scores.push(score)
        }
    }

    scores.sort();

    scores[scores.len() / 2]
}

impl Chunk {
    fn offending_char(&self) -> Option<char> {
        let mut stack = VecDeque::new();

        for ch in self.content.chars() {
            match ch {
                '(' | '[' | '<' | '{' => {
                    stack.push_back(ch);
                }

                ')' if stack.back() == Some(&'(') => {
                    stack.pop_back();
                }

                ']' if stack.back() == Some(&'[') => {
                    stack.pop_back();
                }
                '>' if stack.back() == Some(&'<') => {
                    stack.pop_back();
                }
                '}' if stack.back() == Some(&'{') => {
                    stack.pop_back();
                }

                ')' | ']' | '>' | '}' => return Some(ch),

                _ => unreachable!(),
            };
        }

        None
    }

    fn competion(&self) -> Option<String> {
        let mut stack = VecDeque::new();

        for ch in self.content.chars() {
            match ch {
                '(' | '[' | '<' | '{' => {
                    stack.push_back(ch);
                }

                ')' if stack.back() == Some(&'(') => {
                    stack.pop_back();
                }

                ']' if stack.back() == Some(&'[') => {
                    stack.pop_back();
                }
                '>' if stack.back() == Some(&'<') => {
                    stack.pop_back();
                }
                '}' if stack.back() == Some(&'{') => {
                    stack.pop_back();
                }

                ')' | ']' | '>' | '}' => return None,

                _ => unreachable!(),
            };
        }

        let mut buf = String::default();

        while let Some(s) = stack.pop_back() {
            buf.push(match s {
                '(' => ')',
                '{' => '}',
                '<' => '>',
                '[' => ']',
                _ => unreachable!(),
            });
        }

        Some(buf)
    }
}

impl utils::Parsable for Chunk {
    fn parse(input: &str) -> IResult<&str, Chunk> {
        let content = input.to_string();
        Ok((input, Chunk { content }))
    }
}

#[test]
fn input() {
    let chunk: &Chunk = &utils::test_input_vec()[0];

    assert_eq!(
        *chunk,
        Chunk {
            content: "[({(<(())[]>[[{[]{<()<>>".to_string()
        }
    );
}

#[test]
fn test_offending() {
    let chunk = Chunk {
        content: "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
    };

    assert_eq!(chunk.offending_char(), Some('}'));
}

#[test]
fn test_error_score() {
    let chunks = utils::test_input_vec();

    assert_eq!(error_score(&chunks), 26397);
}

#[test]
fn test_mid_auto_complete() {
    let chunks: Vec<Chunk> = utils::test_input_vec();

    assert_eq!(mid_autocomplete_score(&chunks), 288957);
}
