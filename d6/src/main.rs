use std::fs;

use itertools::izip;

#[derive(Debug)]
enum Operators {
    Plus,
    Multiply,
}

impl From<char> for Operators {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Plus,
            '*' => Self::Multiply,
            c => panic!("unknown operator '{}'", c),
        }
    }
}

impl Operators {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a as u64 + b as u64,
            Self::Multiply => a as u64 * b as u64,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parts = input.split('\n');
    let num_lines = parts
        .to_owned()
        .filter(|line| !(line.contains('+') || line.contains('*')))
        .map(|line| line.chars().collect());
    let ops_line: Vec<char> = parts
        .filter(|line| line.contains('+') || line.contains('*'))
        .next()
        .unwrap()
        .chars()
        .collect();

    num_lines.take(4).zip(ops_line).map(|(a, b, c, d, op)| {});

    println!("{}", res);
}
