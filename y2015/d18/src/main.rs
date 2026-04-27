use std::fmt::{Display, Write};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Swap {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
struct Grid {
    step: u64,
    array_a: [[bool; WIDTH]; HEIGHT],
    array_b: [[bool; WIDTH]; HEIGHT],
    swap: Swap,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_lines(lines: &Vec<&str>) -> Self {
        let mut grid = Self {
            step: 0,
            array_a: [[false; WIDTH]; HEIGHT],
            array_b: [[false; WIDTH]; HEIGHT],
            swap: Swap::A,
            width: WIDTH,
            height: HEIGHT,
        };

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in (**line).char_indices() {
                grid.array_a[y][x] = char == '#';
            }
        }

        grid
    }

    fn get_grid(&self) -> &[[bool; WIDTH]; HEIGHT] {
        match self.swap {
            Swap::A => &self.array_a,
            Swap::B => &self.array_b,
        }
    }

    fn get_mut_grid(&mut self) -> &mut [[bool; WIDTH]; HEIGHT] {
        match self.swap {
            Swap::A => &mut self.array_b,
            Swap::B => &mut self.array_a,
        }
    }

    fn swap(&mut self) {
        match self.swap {
            Swap::A => self.swap = Swap::B,
            Swap::B => self.swap = Swap::A,
        }
    }

    fn get(&self, x: i16, y: i16) -> Option<bool> {
        if x < 0 || y < 0 {
            None
        } else {
            if let Some(row) = self.get_grid().get(y as usize) {
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
                match (x, y) {
                    (0, 0) => self.get_mut_grid()[y][x] = true,
                    (0, 99) => self.get_mut_grid()[y][x] = true,
                    (99, 0) => self.get_mut_grid()[y][x] = true,
                    (99, 99) => self.get_mut_grid()[y][x] = true,
                    _ => {
                        let number_of_neighbours = self.neighbours(x, y);
                        let this = self.get(x as i16, y as i16).unwrap();

                        match (this, number_of_neighbours) {
                            (true, 2 | 3) => self.get_mut_grid()[y][x] = true,
                            (true, _) => self.get_mut_grid()[y][x] = false,
                            (false, 3) => self.get_mut_grid()[y][x] = true,
                            (false, _) => self.get_mut_grid()[y][x] = false,
                        };
                    }
                }
            }
        }

        self.swap();
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
        for line in self.get_grid() {
            for light in line {
                f.write_char(if *light { '#' } else { '.' })?;
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

    let mut grid = Grid::from_lines(&lines);

    println!("{}", grid);
    for _ in 0..100 {
        grid.step();
        println!("{}", grid);
    }

    println!("{}: {}", grid.step, grid.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours_none() {
        let grid = Grid::from_lines(&vec!["......"; 6]);

        assert_eq!(grid.neighbours(0, 0), 0);

        assert_eq!(grid.neighbours(3, 3), 0);
    }

    #[test]
    fn test_neighbours_some() {
        let grid = Grid::from_lines(&vec!["#....."; 6]);

        assert_eq!(grid.neighbours(0, 0), 1);

        assert_eq!(grid.neighbours(3, 3), 0);
    }

    #[test]
    fn test_neighbours_lots() {
        let grid = Grid::from_lines(&vec!["######"; 6]);

        assert_eq!(grid.neighbours(0, 0), 3);

        assert_eq!(grid.neighbours(3, 3), 8);
    }
}
