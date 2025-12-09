use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let parts = value.split(',');
        let mut nums = parts.take(3).map(|i| u64::from_str_radix(i, 10).unwrap());
        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
        }
    }
}

impl Point {
    fn distance_square(&self, other: &Self) -> u64 {
        vec![
            self.x.abs_diff(other.x),
            self.y.abs_diff(other.y),
            self.z.abs_diff(other.z),
        ]
        .into_iter()
        .map(|v| v * v)
        .fold(0, |a, b| a + b)
    }
}

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();

    let points: Vec<Point> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(Point::from)
        .collect();

    let pairs: Vec<(Point, Point)> = combinatorial::Combinations::of_size(points, 2)
        .map(|i| {
            let mut i = i.into_iter();
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect();

    let mut pair_distance: Vec<((Point, Point), u64)> = pairs
        .into_iter()
        .map(|(a, b)| {
            let dist = a.distance_square(&b);
            ((a, b), dist)
        })
        .collect();
    pair_distance.sort_by_key(|((_, _), n)| *n);

    let mut circuits: Vec<HashSet<Point>> = vec![];

    for pair in pair_distance {
        let n = circuits
            .iter_mut()
            .map(|c| match (c.contains(&pair.0.0), c.contains(&pair.0.1)) {
                // TODO: merging
                (true, false) => {
                    c.insert(pair.0.1.to_owned());
                }
                (false, true) => {
                    c.insert(pair.0.0.to_owned());
                }
                (false, false) => {}
                (true, true) => {}
            })
            .count();
        if n == 0 {
            circuits.push(HashSet::from([pair.0.0, pair.0.1]));
        }
    }

    let mut sizes: Vec<usize> = circuits.into_iter().map(|c| c.len()).collect();
    sizes.sort();

    println!("{:?}", sizes);

    let res = sizes.iter().take(3).fold(1, |a, b| a * b);
    println!("{}", res);
}
