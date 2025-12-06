use std::fs;

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
    fn apply(&self, a: u32, b: u64) -> u64 {
        match self {
            Self::Plus => a as u64 + b as u64,
            Self::Multiply => a as u64 * b as u64,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut parts = input.split('\n');
    let (a, b, c, d, o) = (
        parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<u32>>(),
        parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<u32>>(),
        parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<u32>>(),
        parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<u32>>(),
        parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|o| o.len() > 0)
            .map(|o| Operators::from(o.chars().into_iter().next().unwrap()))
            .collect::<Vec<Operators>>(),
    );

    let res: u64 = a
        .into_iter()
        .zip(b)
        .zip(c)
        .zip(d)
        .zip(o)
        .map(|((((a, b), c), d), o)| o.apply(d, o.apply(c, o.apply(a, b as u64))))
        .sum();

    println!("{}", res);
}
