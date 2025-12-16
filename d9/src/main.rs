use std::fmt::Display;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CompressedPoint {
    original: Point,
    compressed: Point,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge<'a> {
    a: &'a Point,
    b: &'a Point,
}

impl<'a> Edge<'a> {
    fn intersects(&self, other: &Self) -> bool {
        self.a.x.min(self.b.x) < other.a.x.max(other.b.x)
            && self.a.x.max(self.b.x) > other.a.x.min(other.b.x)
            && self.a.y.min(self.b.y) < other.a.y.max(other.b.y)
            && self.a.y.max(self.b.y) > other.a.y.min(other.b.y)
    }
}

fn part_1(points: Vec<Point>) {
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

fn part_2(points: Vec<CompressedPoint>) {
    // TODO: add rotation detection
    let mut edges: Vec<Edge> = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(a, b)| Edge { a: a, b: b })
        .collect();
    edges.push(Edge {
        a: points.last().unwrap(),
        b: points.first().unwrap(),
    });

    // TODO: check that each of the four corners are within the 4 closest edges in each direction

    let mut pair_areas: Vec<u64> = Combinations::of_size(points.to_owned(), 2)
        .map(|vec| {
            let mut i = vec.into_iter();
            (i.next().unwrap(), i.next().unwrap())
        })
        .filter(|(a, b)| {
            let rect_points = [a, &Point { x: a.x, y: b.y }, b, &Point { x: b.x, y: a.y }];
            let rect_edges = vec![
                Edge {
                    a: rect_points[0],
                    b: rect_points[1],
                },
                Edge {
                    a: rect_points[1],
                    b: rect_points[2],
                },
                Edge {
                    a: rect_points[2],
                    b: rect_points[3],
                },
                Edge {
                    a: rect_points[3],
                    b: rect_points[0],
                },
            ];
            if true == false {
                println!("{:?}", rect_points);
                println!("{:?}", rect_edges);
                println!(
                    "{:?}",
                    rect_edges
                        .iter()
                        .map(|e| edges
                            .iter()
                            .filter(|other| e.intersects(other))
                            .collect::<Vec<&Edge>>())
                        .collect::<Vec<Vec<&Edge>>>()
                )
            }
            !rect_edges
                .into_iter()
                .any(|e| edges.iter().any(|other| e.intersects(other)))
        })
        .map(|(a, b)| a.rect_area(&b))
        .collect();

    pair_areas.sort();

    println!("{:?}", pair_areas.last().unwrap());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut points: Vec<(Point, Option<u32>, Option<u32>)> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| (Point::from(line), None, None))
        .collect();

    points.sort_by_key(|p| p.0.x);
    points
        .iter_mut()
        .enumerate()
        .map(|(i, (_, x, _))| *x = Some(i as u32))
        .collect::<()>();
    points.sort_by_key(|p| p.0.y);
    points
        .iter_mut()
        .enumerate()
        .map(|(i, (_, _, y))| *y = Some(i as u32))
        .collect::<()>();

    let points: Vec<CompressedPoint> = points
        .into_iter()
        .map(|(p, x, y)| CompressedPoint {
            original: p,
            compressed: Point {
                x: x.unwrap(),
                y: y.unwrap(),
            },
        })
        .collect();

    part_2(points);
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

    #[test]
    fn test_intersect() {
        assert!(
            Edge {
                a: &Point { x: 1, y: 1 },
                b: &Point { x: 1, y: 3 },
            }
            .intersects(&Edge {
                a: &Point { x: 0, y: 2 },
                b: &Point { x: 2, y: 2 }
            })
        );

        assert!(
            Edge {
                a: &Point { x: 1, y: 1 },
                b: &Point { x: 3, y: 1 },
            }
            .intersects(&Edge {
                a: &Point { x: 2, y: 0 },
                b: &Point { x: 2, y: 2 }
            })
        )
    }
}
