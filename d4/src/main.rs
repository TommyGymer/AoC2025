use std::fs;

enum Cell {
    Paper,
    None,
}

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
                        .map(|c| if c == '#' { Cell::Paper } else { Cell::None })
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
                    .map(|(x, _)| {
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
                        .count() as u8
                    })
                    .collect()
            })
            .collect()
    }

    fn get_cell(&self, x: usize, y: usize, i: i8, j: i8) -> Option<&Cell> {
        let x = x.checked_add_signed(i as isize)?;
        let y = y.checked_add_signed(j as isize)?;
        self.data.get(x)?.get(y)
    }
}

fn main() {
    let input = fs::read_to_string("example.txt").unwrap();

    let floor = Floor::from(input);
    println!("{:?}", floor.compute_convolution());
}
