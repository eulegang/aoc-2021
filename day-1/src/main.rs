use std::fs::read_to_string;

fn levels(content: &str) -> Vec<u32> {
    let mut v = Vec::new();
    for line in content.split("\n") {
        if line.is_empty() {
            continue;
        }

        if let Ok(num) = line.parse() {
            v.push(num);
        }
    }

    v
}

fn increases(l: &[u32]) -> u32 {
    if l.len() == 0 {
        return 0;
    }

    let mut level = l[0];
    let mut inc = 0;

    for cur in &l[1..] {
        if *cur > level {
            inc += 1;
        }

        level = *cur;
    }

    inc
}

fn windows(l: &[u32]) -> Vec<(u32, u32, u32)> {
    let mut res = Vec::new();

    for i in 0..l.len() - 2 {
        res.push((l[i], l[i + 1], l[i + 2]));
    }

    res
}

fn main() {
    let content = read_to_string("input").unwrap();
    let l = levels(&content);

    println!("increases: {}", increases(&l));

    let win = windows(&l);
    let blended = win.iter().map(|(a, b, c)| a + b + c).collect::<Vec<_>>();

    println!("windowed increases: {}", increases(&blended));
}

#[test]
fn test_input_model() {
    let content = read_to_string("test-input").unwrap();
    let l = levels(&content);

    assert_eq!(&l, &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
}

#[test]
fn test_answer() {
    let content = read_to_string("test-input").unwrap();
    let l = levels(&content);
    assert_eq!(increases(&l), 7);
}

#[test]
fn test_second() {
    let content = read_to_string("test-input").unwrap();
    let l = levels(&content);
    let win = windows(&l);

    let x = win.iter().map(|(a, b, c)| a + b + c).collect::<Vec<_>>();
    assert_eq!(increases(&x), 5);
}
