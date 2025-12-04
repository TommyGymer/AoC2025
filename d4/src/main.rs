use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Cell {
    Paper,
    None,
}

#[derive(Debug)]
struct Floor {
    data: Vec<Vec<Cell>>,
}

impl From<String> for Floor {
    fn from(value: String) -> Self {
        Floor {
            data: value
                .split('\n')
                .map(|l| {
                    l.chars()
                        .map(|c| if c == '@' { Cell::Paper } else { Cell::None })
                        .collect::<Vec<Cell>>()
                })
                .filter(|a| a.len() > 0)
                .collect(),
        }
    }
}

impl Floor {
    fn compute_convolution(&self) -> Vec<Vec<u8>> {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if *c == Cell::Paper {
                            [
                                (1, 0),
                                (-1, 0),
                                (0, 1),
                                (0, -1),
                                (-1, 1),
                                (1, -1),
                                (1, 1),
                                (-1, -1),
                            ]
                            .iter()
                            .filter_map(|(i, j)| self.get_cell(x, y, *i, *j))
                            .filter(|c| **c == Cell::Paper)
                            .count() as u8
                        } else {
                            8
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn get_cell(&self, x: usize, y: usize, i: i8, j: i8) -> Option<&Cell> {
        let x = x.checked_add_signed(i as isize)?;
        let y = y.checked_add_signed(j as isize)?;
        self.data.get(y)?.get(x)
    }

    fn do_remove(&self, conv: Vec<Vec<u8>>) -> Self {
        Self {
            data: self
                .data
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, cell)| {
                            if *conv.get(x).unwrap().get(y).unwrap() < 4 {
                                Cell::None
                            } else {
                                *cell
                            }
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut floor = Floor::from(input);
    let mut count = 0;
    let mut should_continue = true;
    while should_continue {
        let conv = floor.compute_convolution();
        let res: usize = conv
            .iter()
            .map(|row| row.iter().filter(|cell| **cell < 4).count())
            .sum();
        count += res;
        should_continue = res > 0;
        floor = floor.do_remove(conv);
    }

    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(5usize.checked_add_signed(1isize), Some(6));
        assert_eq!(5usize.checked_add_signed(-1isize), Some(4));
        assert_eq!(0usize.checked_add_signed(-1isize), None);
    }
}
