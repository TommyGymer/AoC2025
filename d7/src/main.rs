use std::{fmt::Display, fs};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Splitter,
    Beam,
    Start,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ".",
                Self::Splitter => "^",
                Self::Beam => "|",
                Self::Start => "S",
            }
        )
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Empty,
            '^' => Self::Splitter,
            '|' => Self::Beam,
            c => panic!("unknown value {} in cell", c),
        }
    }
}

#[derive(Debug)]
struct State {
    cells: Vec<Vec<Cell>>,
    splits: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self
            .cells
            .iter()
            .map(|line| {
                writeln!(
                    f,
                    "{}",
                    line.iter().map(|c| format!("{}", c)).collect::<String>()
                )
                .unwrap()
            })
            .collect::<()>();
        Ok(())
    }
}

impl State {
    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        match self.cells.get(y) {
            Some(row) => match row.get(x) {
                Some(cell) => Some(cell.to_owned()),
                None => None,
            },
            None => None,
        }
    }

    fn update(&mut self, x: usize, y: usize, value: Cell) {
        match self.cells.get_mut(y) {
            Some(row) => match row.get_mut(x) {
                Some(cell) => *cell = value,
                None => {}
            },
            None => {}
        }
    }

    fn do_updates(&mut self, x: usize, y: usize) {
        match self.get(x, y) {
            None => {}
            Some(cell) => match cell {
                Cell::Empty => {}
                Cell::Start => match self.get(x, y + 1) {
                    None => {}
                    Some(other) => match other {
                        Cell::Empty => self.update(x, y + 1, Cell::Beam),
                        Cell::Start => panic!("start should only be at the top of the layout"),
                        Cell::Beam => {}
                        Cell::Splitter => {}
                    },
                },
                Cell::Splitter => {}
                Cell::Beam => match self.get(x, y + 1) {
                    None => {}
                    Some(other) => match other {
                        Cell::Empty => {
                            self.update(x, y + 1, Cell::Beam);
                            match self.get(x, y + 2) {
                                None => {}
                                Some(extend) => match extend {
                                    Cell::Empty => self.update(x, y + 2, Cell::Beam),
                                    _ => {}
                                },
                            }
                        }
                        Cell::Splitter => {
                            self.splits += 1;
                            match self.get(x - 1, y + 1) {
                                None => {}
                                Some(left) => match left {
                                    Cell::Empty => {
                                        self.update(x - 1, y + 1, Cell::Beam);
                                        match self.get(x - 1, y + 2) {
                                            None => {}
                                            Some(extend) => match extend {
                                                Cell::Empty => {
                                                    self.update(x - 1, y + 2, Cell::Beam)
                                                }
                                                _ => {}
                                            },
                                        }
                                    }
                                    Cell::Splitter | Cell::Start | Cell::Beam => {}
                                },
                            };
                            match self.get(x + 1, y + 1) {
                                None => {}
                                Some(left) => match left {
                                    Cell::Empty => {
                                        self.update(x + 1, y + 1, Cell::Beam);
                                        match self.get(x + 1, y + 2) {
                                            None => {}
                                            Some(extend) => match extend {
                                                Cell::Empty => {
                                                    self.update(x + 1, y + 2, Cell::Beam)
                                                }
                                                _ => {}
                                            },
                                        }
                                    }
                                    Cell::Splitter | Cell::Start | Cell::Beam => {}
                                },
                            }
                        }
                        Cell::Start => panic!("start should only be at the top of the layout"),
                        Cell::Beam => {}
                    },
                },
            },
        }
    }
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut layout: State = State {
        cells: input
            .split('\n')
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Vec<Cell>>()
            })
            .collect(),
        splits: 0,
    };

    let mut prev_splits = 0;
    for y in 0..layout.cells.len() - 2 {
        for x in 0..layout.cells.first().unwrap().len() {
            layout.do_updates(x, y);
            if layout.splits != prev_splits {
                println!("----------------------");
                println!("({}, {})", x, y);
                println!("{}", layout);
                println!("splits: {}", layout.splits);
                println!("----------------------");
                prev_splits = layout.splits;
            }
        }
    }

    println!("{}", layout.splits);
}

fn main() {
    let input = fs::read_to_string("example.txt").unwrap();
}
