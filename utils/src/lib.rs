use std::fs::read_to_string;

pub trait Parsable: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

pub fn parse_file_vec<T: Parsable>(file: &str) -> Vec<T> {
    let content = read_to_string(file).expect(&format!("reading {}", file));
    let mut res = Vec::new();

    for line in content.lines() {
        let (_, elem) = T::parse(&line).expect("unable to parse");
        res.push(elem);
    }

    res
}

pub fn parse_file<T: Parsable>(file: &str) -> T {
    let content = read_to_string(file).expect(&format!("reading {}", file));
    T::parse(&content).expect("unable to parse").1
}

pub fn test_input_vec<T: Parsable>() -> Vec<T> {
    parse_file_vec("test.input")
}

pub fn input_vec<T: Parsable>() -> Vec<T> {
    parse_file_vec("input")
}

pub fn test_input<T: Parsable>() -> T {
    parse_file("test.input")
}

pub fn input<T: Parsable>() -> T {
    parse_file("input")
}

pub struct Bound {
    cur: usize,
    len: usize,
}

impl Iterator for Bound {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.len == 0 {
            return None;
        }

        let res = self.cur;

        self.cur += 1;
        self.len -= 1;

        Some(res)
    }
}

pub fn neigh(i: usize, cap: usize) -> Bound {
    let mut len = 3;
    let mut cur = 0;

    if let Some(s) = i.checked_sub(1) {
        cur = s;
    } else {
        len -= 1;
    }

    len = len.min(cap - cur);

    Bound { cur, len }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xyz() {
       assert_eq!(vec![0, 1], neigh(0, 10).collect::<Vec<_>>());
       assert_eq!(vec![4, 5, 6], neigh(5, 10).collect::<Vec<_>>());
       assert_eq!(vec![8, 9], neigh(9, 10).collect::<Vec<_>>());
    }
}
