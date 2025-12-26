use std::{array, collections::VecDeque, fs};

fn insert_select_n_from_k(battery: &Vec<u8>, n: usize) -> u64 {
    (0..n)
        .fold((0usize, 0u64), |acc, i| {
            let (start, v) = battery.as_slice()[acc.0..(battery.len() - (n - 1) + i) as usize]
                .into_iter()
                .enumerate()
                .reduce(|(acc_i, acc_v), (index, value)| {
                    if value <= acc_v {
                        (acc_i, acc_v)
                    } else {
                        (index, value)
                    }
                })
                .unwrap();
            (acc.0 + start + 1, acc.1 * 10 + *v as u64)
        })
        .1
}

fn bucket_select_n_from_k(battery: &Vec<u8>, n: usize) -> u64 {
    // sort
    let mut buckets: [VecDeque<(usize, u8)>; 9] =
    // NOTE: there is no noticable performance decrease with this capacity
        array::from_fn(|_| VecDeque::with_capacity(battery.len() / 9));
    let mut battery_iter = battery.into_iter().enumerate();
    let _ = battery_iter
        .by_ref()
        .take(battery.len() - n)
        .map(|(i, b)| buckets[*b as usize - 1].push_back((i, *b)))
        .collect::<()>();

    // select
    (0..n)
        .fold((0usize, 0u64), |acc, _| {
            // push next battery into buckets to maintain buffer at end
            let next_battery = battery_iter.next().unwrap();
            buckets[*next_battery.1 as usize - 1].push_back((next_battery.0, *next_battery.1));

            let mut selected_battery = None;

            while selected_battery.is_none() {
                let next = if let Some(item) = buckets[8].pop_front() {
                    item
                } else if let Some(item) = buckets[7].pop_front() {
                    item
                } else if let Some(item) = buckets[6].pop_front() {
                    item
                } else if let Some(item) = buckets[5].pop_front() {
                    item
                } else if let Some(item) = buckets[4].pop_front() {
                    item
                } else if let Some(item) = buckets[3].pop_front() {
                    item
                } else if let Some(item) = buckets[2].pop_front() {
                    item
                } else if let Some(item) = buckets[1].pop_front() {
                    item
                } else if let Some(item) = buckets[0].pop_front() {
                    item
                } else {
                    unreachable!("someone did a silly");
                };
                if next.0 >= acc.0 {
                    selected_battery = Some(next);
                }
            }
            let selected_battery = selected_battery.expect("someone did a silly");

            (selected_battery.0, acc.1 * 10 + selected_battery.1 as u64)
        })
        .1
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total: u64 = input
        .split('\n')
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect::<Vec<u8>>()
        })
        .filter(|l| l.len() > 0)
        .map(|battery| bucket_select_n_from_k(&battery, 12))
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
