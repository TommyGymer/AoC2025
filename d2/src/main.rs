use std::{fs, iter::repeat_n, ops::Range};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let res: u64 = input
        .split('\n')
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            let mut sides = s.split('-');
            Range {
                start: usize::from_str_radix(sides.next().unwrap(), 10).unwrap(),
                end: usize::from_str_radix(sides.next().unwrap(), 10).unwrap(),
            }
        })
        .map(|r| {
            r.filter_map(|i| {
                let as_string = format!("{}", i);

                if is_repeating(&as_string) {
                    Some(i as u64)
                } else {
                    None
                }
            })
            .sum::<u64>()
        })
        .sum();

    println!("{}", res)
}

fn is_repeat(s: String) -> bool {
    let mut string = s.to_owned();
    let half = string.split_off(s.len() / 2);
    half == string
}

fn is_repeating(s: &str) -> bool {
    (1..=s.len() / 2)
        .map(|i| s.split_at(i).0)
        .filter(|part| s.len() % part.len() == 0)
        .map(|part| {
            let repeats = s.len() / part.len();
            repeats > 1 && repeat_n(part, repeats).collect::<String>() == s
        })
        .any(|b| b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!is_repeating("1"))
    }

    #[test]
    fn test_11() {
        assert!(is_repeating("11"))
    }

    #[test]
    fn test_101() {
        assert!(!is_repeating("101"))
    }
}
