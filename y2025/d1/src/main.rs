use std::{
    fs,
    iter::{self, repeat_n},
};

enum Rotation {
    R(u64),
    L(u64),
}

#[derive(Debug)]
struct Dial {
    pub pointer: u64,
    pub zero_count: u64,
}

impl Dial {
    fn rotate(self, rotation: &Rotation) -> Self {
        let mut pointer: i64 = self.pointer as i64;
        let mut zero_count = self.zero_count;
        match rotation {
            Rotation::R(n) => {
                pointer += *n as i64;
            }
            Rotation::L(n) => {
                pointer -= *n as i64;
            }
        }
        let mod_pointer = if pointer < 0 {
            100 + (pointer % 100)
        } else {
            pointer % 100
        };

        if mod_pointer == 0 {
            zero_count += 1;
        }
        Self {
            pointer: mod_pointer as u64,
            zero_count,
        }
    }

    fn iter_rotate(self, rotation: &Rotation) -> Self {
        match rotation {
            Rotation::R(n) => {
                repeat_n(1, *n as usize).fold(self, |acc, _| acc.rotate(&Rotation::R(1)))
            }
            Rotation::L(n) => {
                repeat_n(1, *n as usize).fold(self, |acc, _| acc.rotate(&Rotation::L(1)))
            }
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            pointer: 50,
            zero_count: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let res: Dial = input
        .split('\n')
        .filter_map(|l| {
            if l.starts_with(|c| c == 'L' || c == 'R') {
                if l.starts_with('L') {
                    Some(Rotation::L(
                        u64::from_str_radix(l.trim_start_matches('L'), 10).unwrap_or(0),
                    ))
                } else {
                    Some(Rotation::R(
                        u64::from_str_radix(l.trim_start_matches('R'), 10).unwrap_or(0),
                    ))
                }
            } else {
                None
            }
        })
        .fold(Dial::default(), |acc, r| acc.iter_rotate(&r));

    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_n;

    use super::*;

    #[test]
    fn test_default() {
        let dial = Dial::default();

        assert_eq!(dial.pointer, 50);
        assert_eq!(dial.zero_count, 0);
    }

    #[test]
    fn test_rot_right() {
        let mut dial = Dial::default();

        dial = dial.iter_rotate(&Rotation::R(1));
        assert_eq!(dial.pointer, 51);
        assert_eq!(dial.zero_count, 0);

        dial = dial.iter_rotate(&Rotation::R(48));
        assert_eq!(dial.pointer, 99);
        assert_eq!(dial.zero_count, 0);

        dial = dial.iter_rotate(&Rotation::R(1));
        assert_eq!(dial.pointer, 0);
        assert_eq!(dial.zero_count, 1);

        dial = dial.iter_rotate(&Rotation::R(101));
        assert_eq!(dial.pointer, 1);
        assert_eq!(dial.zero_count, 2);

        dial = dial.iter_rotate(&Rotation::R(201));
        assert_eq!(dial.pointer, 2);
        assert_eq!(dial.zero_count, 4);
    }

    #[test]
    fn test_rot_left() {
        let mut dial = Dial::default();

        dial = dial.iter_rotate(&Rotation::L(1));
        assert_eq!(dial.pointer, 49);
        assert_eq!(dial.zero_count, 0);

        dial = dial.iter_rotate(&Rotation::L(49));
        assert_eq!(dial.pointer, 0);
        assert_eq!(dial.zero_count, 1);

        dial = dial.iter_rotate(&Rotation::L(101));
        assert_eq!(dial.pointer, 99);
        assert_eq!(dial.zero_count, 2);

        dial = dial.iter_rotate(&Rotation::L(201));
        assert_eq!(dial.pointer, 98);
        assert_eq!(dial.zero_count, 4);
    }
}
