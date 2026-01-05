use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

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

impl CompressedPoint {
    fn rect_area(&self, other: &CompressedPoint) -> u64 {
        self.original.rect_area(&other.original)
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

#[derive(Debug, Clone, Copy)]
enum Space {
    Outside,
    Inside,
    Boundary,
}

impl Space {
    fn is_polygon(&self) -> bool {
        match self {
            Self::Outside => false,
            Self::Inside => true,
            Self::Boundary => true,
        }
    }
}

fn fill(point: Point, space: &mut Vec<Vec<Space>>) {
    let mut stack: Vec<Point> = vec![point];
    let mut should_continue = true;
    while should_continue {
        if let Some(point) = stack.pop() {
            let space_len = space.len() as u32;
            match space.get(point.y as usize) {
                None => {}
                Some(row) => {
                    let row_len = row.len() as u32;
                    match row.get(point.x as usize) {
                        None => {}
                        Some(cell) => match *cell {
                            Space::Outside => {}
                            Space::Boundary => {}
                            Space::Inside => {
                                *space
                                    .get_mut(point.y as usize)
                                    .unwrap()
                                    .get_mut(point.x as usize)
                                    .unwrap() = Space::Outside;
                                let _ = vec![(0i32, 1i32), (0, -1), (1, 0), (-1, 0)]
                                    .into_iter()
                                    .map(|(x, y)| {
                                        (
                                            point.x.checked_add_signed(x),
                                            point.y.checked_add_signed(y),
                                        )
                                    })
                                    .filter(|(x, y)| x.is_some() && y.is_some())
                                    .map(|(x, y)| (x.unwrap(), y.unwrap()))
                                    .filter(|(x, y)| *x < row_len && *y < space_len)
                                    .map(|(x, y)| stack.push(Point { x, y }))
                                    .collect::<()>();
                            }
                        },
                    }
                }
            }
        } else {
            should_continue = false;
        }
    }
}

fn part_2(points: Vec<CompressedPoint>) -> u64 {
    // construct polygon
    let mut max_x = 0;
    let mut max_y = 0;
    for point in &points {
        if point.compressed.x > max_x {
            max_x = point.compressed.x;
        }
        if point.compressed.y > max_y {
            max_y = point.compressed.y;
        }
    }

    let mut space = vec![vec![Space::Inside; max_x as usize + 3]; max_y as usize + 3];

    // boundary
    let mut edges: Vec<Edge> = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(a, b)| Edge {
            a: &a.compressed,
            b: &b.compressed,
        })
        .collect();
    edges.push(Edge {
        a: &points.last().unwrap().compressed,
        b: &points.first().unwrap().compressed,
    });

    for edge in edges {
        if edge.a.x == edge.b.x {
            // goes along y
            let min = edge.a.y.min(edge.b.y);
            for i in 0..=edge.a.y.abs_diff(edge.b.y) {
                space[(min + i) as usize][edge.a.x as usize] = Space::Boundary;
            }
        } else if edge.a.y == edge.b.y {
            // goes along x
            let min = edge.a.x.min(edge.b.x);
            for i in 0..=edge.a.x.abs_diff(edge.b.x) {
                space[edge.a.y as usize][(min + i) as usize] = Space::Boundary;
            }
        } else {
            unreachable!("this shouldn't happen in the input data")
        }
    }

    // fill outside
    fill(Point { x: 0, y: 0 }, &mut space);
    println!("{:?}", space);

    // construct all pairs
    let mut pair_areas: Vec<u64> = Combinations::of_size(points, 2)
        .map(|vec| {
            let mut i = vec.into_iter();
            (i.next().unwrap(), i.next().unwrap())
        })
        .filter(|(a, b)| {
            let rect_points = [
                a.compressed,
                Point {
                    x: a.compressed.x,
                    y: b.compressed.y,
                },
                b.compressed,
                Point {
                    x: b.compressed.x,
                    y: a.compressed.y,
                },
            ];
            let rect_edges = vec![
                Edge {
                    a: &rect_points[0],
                    b: &rect_points[1],
                },
                Edge {
                    a: &rect_points[1],
                    b: &rect_points[2],
                },
                Edge {
                    a: &rect_points[2],
                    b: &rect_points[3],
                },
                Edge {
                    a: &rect_points[3],
                    b: &rect_points[0],
                },
            ];

            rect_edges.into_iter().all(|edge| {
                if edge.a.x == edge.b.x {
                    // goes along y
                    let min = edge.a.y.min(edge.b.y);
                    (0..=edge.a.y.abs_diff(edge.b.y))
                        .map(|i| space[(min + i) as usize][edge.a.x as usize])
                        .all(|s| s.is_polygon())
                } else if edge.a.y == edge.b.y {
                    // goes along x
                    let min = edge.a.x.min(edge.b.x);
                    (0..=edge.a.x.abs_diff(edge.b.x))
                        .map(|i| space[edge.a.y as usize][(min + i) as usize])
                        .all(|s| s.is_polygon())
                } else {
                    unreachable!("this shouldn't happen in the input data")
                }
            })
        })
        .map(|(a, b)| a.rect_area(&b))
        .collect();

    pair_areas.sort();

    *pair_areas.last().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let points: Vec<Point> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| Point::from(line))
        .collect();

    let xs: HashSet<u32> = points.iter().map(|p| p.x).collect();
    let mut xs: Vec<u32> = xs.into_iter().collect();
    xs.sort();
    let xs: HashMap<u32, u32> = xs
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, (i + 1) as u32))
        .collect();

    let ys: HashSet<u32> = points.iter().map(|p| p.y).collect();
    let mut ys: Vec<u32> = ys.into_iter().collect();
    ys.sort();
    let ys: HashMap<u32, u32> = ys
        .into_iter()
        .enumerate()
        .map(|(i, y)| (y, (i + 1) as u32))
        .collect();

    let points: Vec<CompressedPoint> = points
        .into_iter()
        .map(|p| CompressedPoint {
            original: p,
            compressed: Point {
                x: *xs.get(&p.x).unwrap(),
                y: *ys.get(&p.y).unwrap(),
            },
        })
        .collect();

    let res = part_2(points);

    println!("{}", res);
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
