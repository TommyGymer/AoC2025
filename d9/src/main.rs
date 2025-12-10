use combinatorial::Combinations;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn rect_area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) as u64 * (self.y.abs_diff(other.y) + 1) as u64
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut values = value
            .split(',')
            .filter_map(|part| match u32::from_str_radix(part, 10) {
                Ok(v) => Some(v),
                Err(_) => None,
            });

        Self {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let points: Vec<Point> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| Point::from(line))
        .collect();

    let mut pair_areas: Vec<u64> = Combinations::of_size(points, 2)
        .map(|vec| {
            let mut i = vec.into_iter();
            (i.next().unwrap(), i.next().unwrap())
        })
        .map(|(a, b)| a.rect_area(&b))
        .collect();

    pair_areas.sort();

    println!("{:?}", pair_areas.last().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect_area() {
        let a = Point { x: 2, y: 5 };
        let b = Point { x: 11, y: 1 };

        assert_eq!(a.rect_area(&b), 50);
    }
}
