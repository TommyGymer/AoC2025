use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(',').collect();

        Point {
            x: usize::from_str_radix(parts.get(0).unwrap(), 10).unwrap(),
            y: usize::from_str_radix(parts.get(1).unwrap(), 10).unwrap(),
        }
    }
}

fn parse_range(string: &str) -> (Point, Point) {
    let parts: Vec<&str> = string.trim().split(" through ").collect();

    (
        Point::from(*parts.get(0).unwrap()),
        Point::from(*parts.get(1).unwrap()),
    )
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut space = [[false; 1000]; 1000];

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    for line in lines {
        if let Some(rem) = line.strip_prefix("turn on ") {
            let (a, b) = parse_range(rem);

            for x in a.x.min(b.x)..a.x.max(b.x) {
                for y in a.y.min(b.y)..a.y.max(b.y) {
                    *space.get_mut(x).unwrap().get_mut(y).unwrap() = true;
                }
            }
        } else if let Some(rem) = line.strip_prefix("turn off ") {
            let (a, b) = parse_range(rem);

            for x in a.x.min(b.x)..a.x.max(b.x) {
                for y in a.y.min(b.y)..a.y.max(b.y) {
                    *space.get_mut(x).unwrap().get_mut(y).unwrap() = false;
                }
            }
        } else if let Some(rem) = line.strip_prefix("toggle ") {
            let (a, b) = parse_range(rem);

            for x in a.x.min(b.x)..a.x.max(b.x) {
                for y in a.y.min(b.y)..a.y.max(b.y) {
                    *space.get_mut(x).unwrap().get_mut(y).unwrap();
                }
            }
        } else {
            unreachable!()
        }
    }
}
