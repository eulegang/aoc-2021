use std::fs::read_to_string;

pub trait Parsable: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

fn parse_file_vec<T: Parsable>(file: &str) -> Vec<T> {
    let content = read_to_string(file).expect(&format!("reading {}", file));
    let mut res = Vec::new();

    for line in content.lines() {
        let (_, elem) = T::parse(&line).expect("unable to parse");
        res.push(elem);
    }

    res
}

fn parse_file<T: Parsable>(file: &str) -> T {
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
