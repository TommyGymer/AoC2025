use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let parts = value.split(',');
        let mut nums = parts.take(3).map(|i| {
            println!("{}", i);
            u64::from_str_radix(i, 10).unwrap()
        });
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

    let pair_distance: HashMap<(Point, Point), u64> = pairs
        .into_iter()
        .map(|(a, b)| {
            let dist = a.distance_square(&b);
            ((a, b), dist)
        })
        .collect();
}
