use std::fmt::{Display, Write};

const WIDTH: usize = 6;
const HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy)]
struct Grid {
    step: u64,
    array: [[bool; WIDTH]; HEIGHT],
    width: usize,
    height: usize,
}

impl Grid {
    fn from_lines(lines: &Vec<&str>) -> Self {
        let mut grid = Self {
            step: 0,
            array: [[false; WIDTH]; HEIGHT],
            width: WIDTH,
            height: HEIGHT,
        };

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in (**line).char_indices() {
                grid.array[y][x] = char == '#';
            }
        }

        grid
    }

    fn get(&self, x: i16, y: i16) -> Option<bool> {
        if x < 0 || y < 0 {
            None
        } else {
            if let Some(row) = self.array.get(y as usize) {
                row.get(x as usize).cloned()
            } else {
                None
            }
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for (dx, dy) in [
            (1i16, 1i16),
            (1, 0),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (0, -1),
            (1, -1),
            (-1, 1),
        ] {
            if let Some(light) = self.get(x as i16 + dx, y as i16 + dy) {
                if light {
                    count += 1;
                }
            }
        }

        count
    }

    fn step(&mut self) {
        self.step += 1;

        for x in 0..self.width {
            for y in 0..self.height {
                let number_of_neighbours = self.neighbours(x, y);
                let this = self.get(x as i16, y as i16).unwrap();
                match (this, number_of_neighbours) {
                    (true, 2 | 3) => self.array[y][x] = true,
                    (true, _) => self.array[y][x] = false,
                    (false, 3) => self.array[y][x] = true,
                    _ => self.array[y][x] = false,
                }
            }
        }
    }

    fn count(&self) -> u64 {
        let mut count = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x as i16, y as i16).unwrap_or(false) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.array {
            for light in line {
                f.write_char(if light { '#' } else { ' ' })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn main() {
    let input: String = std::fs::read_to_string("example.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut grid = Grid::from_lines(&lines);

    println!("{}", grid);
    for _ in 0..4 {
        grid.step();
        println!("{}", grid);
    }

    println!("{}: {}", grid.step, grid.count());
}
