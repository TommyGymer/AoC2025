#[derive(Debug, Clone, Copy)]
struct Shape {
    shape: [[bool; 3]; 3],
}

#[derive(Debug)]
struct Area {
    size: (u32, u32),
    shapes: Vec<u8>,
}

impl From<&Vec<&str>> for Shape {
    fn from(value: &Vec<&str>) -> Self {
        let mut shape = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                shape[i][j] = if value
                    .get(i)
                    .unwrap()
                    .chars()
                    .into_iter()
                    .collect::<Vec<char>>()
                    .get(j)
                    .unwrap()
                    == &'#'
                {
                    true
                } else {
                    false
                }
            }
        }
        Self { shape }
    }
}

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();

    let sections = input.split("\n\n");
    // NOTE: both inputs have 6 shapes
    let mut shapes: Vec<Shape> = Vec::with_capacity(6);
    let mut areas: Vec<Area> = Vec::new();

    let _ = sections
        .into_iter()
        .filter(|s| s.len() > 0)
        .map(|section| {
            let parts = section.split('\n');
            if section.contains('x') {
                // handle area
                areas = parts
                    .filter(|l| l.len() > 0)
                    .map(|line| {
                        let mut bits = line.split(": ");
                        let mut xy = bits.next().unwrap().split('x');
                        let (x, y) = (
                            u32::from_str_radix(xy.next().unwrap(), 10).unwrap(),
                            u32::from_str_radix(xy.next().unwrap(), 10).unwrap(),
                        );
                        let shapes = bits
                            .next()
                            .unwrap()
                            .split(' ')
                            .map(|s| u8::from_str_radix(s, 10).unwrap())
                            .collect();

                        Area {
                            shapes,
                            size: (x, y),
                        }
                    })
                    .collect();
            } else {
                // shapes are discovered in order, so we can ignore the index line
                shapes.push(Shape::from(&parts.skip(1).take(3).collect::<Vec<&str>>()));
            }
        })
        .collect::<()>();

    println!("{:?}", shapes);
    println!("{:?}", areas);

    let res: u32 = areas.into_iter().map(|area| {}).sum();

    println!("{}", res)
}
