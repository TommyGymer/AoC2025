use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    let input = std::fs::read_to_string("input.txt").unwrap();

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
    let mut last = None;

    for pair in pair_distance.iter() {
        let mut found: Vec<(usize, &mut HashSet<Point>)> = circuits
            .iter_mut()
            .enumerate()
            .filter(|(_, c)| c.contains(&pair.0.0) || c.contains(&pair.0.1))
            .collect();

        let remove = match found.len() {
            0 => {
                circuits.push(HashSet::from([pair.0.0, pair.0.1]));
                None
            }
            // NOTE: this will only add one of the two points as one is already contained
            1 => {
                found.first_mut().unwrap().1.extend([pair.0.0, pair.0.1]);
                None
            }
            2 => {
                let update = found.remove(1);
                found
                    .first_mut()
                    .unwrap()
                    .1
                    .extend(update.1.to_owned().into_iter());
                Some(update.0)
            }
            i => unreachable!("how is it {}", i),
        };
        match remove {
            None => {}
            Some(remove) => {
                circuits.swap_remove(remove);
            }
        }
        if circuits.len() == 1 && circuits.first().unwrap().len() == 1000 && last.is_none() {
            last = Some(pair);
            println!("{:?}", pair);
        }
    }

    let mut sizes: Vec<usize> = circuits.into_iter().map(|c| c.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));

    println!("{:?}", sizes);

    let res = sizes.iter().take(3).fold(1, |a, b| a * b);
    println!("{}", res);
}
