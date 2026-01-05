use core::panic;
use std::fs;

use itertools::izip;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum Operators {
    Plus,
    Multiply,
    #[default]
    None,
}

impl From<char> for Operators {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Plus,
            '*' => Self::Multiply,
            ' ' => Self::None,
            c => panic!("unknown operator '{}'", c),
        }
    }
}

impl Operators {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a as u64 + b as u64,
            Self::Multiply => {
                if a == 0 {
                    1 * b as u64
                } else {
                    a as u64 * b as u64
                }
            }
            Self::None => panic!("cannot apply this operator"),
        }
    }
}

#[derive(Debug, Default)]
struct FoldState {
    problem_op: Operators,
    problem_total: u64,
    total: u64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parts = input.split('\n');
    let mut num_lines = parts
        .to_owned()
        .filter(|line| !(line.contains('+') || line.contains('*')))
        .map(|line| line.chars().collect::<Vec<char>>());
    let ops_line: Vec<char> = parts
        .filter(|line| line.contains('+') || line.contains('*'))
        .next()
        .unwrap()
        .chars()
        .collect();

    let res = izip!(
        num_lines.next().unwrap(),
        num_lines.next().unwrap(),
        num_lines.next().unwrap(),
        num_lines.next().unwrap(),
        ops_line
    )
    .map(|(a, b, c, d, op)| {
        let n = vec![a, b, c, d]
            .into_iter()
            .filter_map(|c| match u64::from_str_radix(&c.to_string(), 10) {
                Ok(i) => Some(i),
                Err(_) => None,
            })
            .fold(0, |acc, v| acc * 10 + v);

        let op = Operators::from(op);
        if n == 0 && op == Operators::None {
            None
        } else {
            Some((n, op))
        }
    })
    .fold(FoldState::default(), |state, value| {
        println!("{:?}", state);
        println!("{:?}", value);
        match value {
            None => FoldState {
                total: state.total + state.problem_total,
                problem_total: 0,
                problem_op: Operators::default(),
            },
            Some((n, op)) => {
                if op == Operators::None {
                    FoldState {
                        problem_op: state.problem_op,
                        problem_total: state.problem_op.apply(state.problem_total, n),
                        total: state.total,
                    }
                } else {
                    FoldState {
                        problem_op: op,
                        problem_total: op.apply(state.problem_total, n),
                        total: state.total,
                    }
                }
            }
        }
    });

    println!("{}", res.total);
}
