use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
struct Grid {
    step: u64,
    array: [[bool; 100]; 100],
}

impl Grid {
    fn from_lines(lines: &Vec<&str>) -> Self {
        let mut grid = Self {
            step: 0,
            array: [[false; 100]; 100],
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
        for (dx, dy) in [(1i16, 1i16), (1, 0), (0, 1), (-1, -1), (-1, 0), (0, -1), (1, -1), (-1, 1)] {
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

        for x in 0..100 {
            for y in 0..100 {
                self.array[y][x] = 
            }
        }
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
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let grid = Grid::from_lines(&lines);

    println!("{}", grid);
}
