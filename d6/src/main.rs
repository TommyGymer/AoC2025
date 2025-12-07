use std::fs;

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
            c => panic!("unknown operator {}", c),
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
    let num_lines: Vec<Vec<char>> = parts
        .to_owned()
        .filter(|line| !(line.contains('+') || line.contains('*')))
        .map(|line| line.chars().collect())
        .collect();
    let ops_line: Vec<char> = parts
        .filter(|line| line.contains('+') || line.contains('*'))
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut res = 0;
    let mut partial_res = 0;
    let mut i = 0;
    while i < ops_line.len() {
        let op = Operators::from(*ops_line.get(i).unwrap());
        partial_res = num_lines
            .iter()
            .map(|line| u64::from_str_radix(&line.get(i).unwrap().to_string(), 10).unwrap())
            .reduce(|a, b| a * 10 + b)
            .unwrap();
    }
}
