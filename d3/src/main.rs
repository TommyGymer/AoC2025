use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total: u64 = input
        .split('\n')
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u64)
                .collect::<Vec<u64>>()
        })
        .filter(|l| l.len() > 0)
        .map(|battery| {
            let res = (0..12)
                .fold((0, 0), |acc, i| {
                    let (start, v) = battery.as_slice()[acc.0..(battery.len() - 11 + i) as usize]
                        .into_iter()
                        .enumerate()
                        .rev()
                        .reduce(|(acc_i, acc_v), (index, value)| {
                            if value < acc_v {
                                (acc_i, acc_v)
                            } else {
                                (index, value)
                            }
                        })
                        .unwrap();
                    (acc.0 + start + 1, acc.1 * 10 + v)
                })
                .1;
            res
        })
        .sum();

    println!("{}", total)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_to_u32() {
        assert_eq!('5'.to_digit(10).unwrap(), 5)
    }

    #[test]
    fn test_char_to_u64() {
        assert_eq!('5'.to_digit(10).unwrap() as u64, 5)
    }
}
