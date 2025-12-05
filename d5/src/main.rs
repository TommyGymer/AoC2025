use std::{fs, ops::Range};

use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let (top, bottom) = (sections.next().unwrap(), sections.next().unwrap());

    let mut largest = 0;
    let ranges: Vec<Range<usize>> = top
        .split('\n')
        .map(|line| {
            let mut parts = line.split('-').filter_map(|num| {
                if let Ok(i) = usize::from_str_radix(num, 10) {
                    Some(i)
                } else {
                    None
                }
            });
            let start = parts.next().unwrap();
            let end = parts.next().unwrap() + 1;
            if largest < end {
                largest = end
            }
            (start..end)
        })
        .collect();

    let res = (0..=largest)
        .par_bridge()
        .filter(|&i| {
            if i % 1000000 == 0 {
                println!(
                    "checking {} of {}: {}",
                    i,
                    largest,
                    (i as f64) / (largest as f64)
                );
            }
            check_item(i, &ranges)
        })
        .count();

    println!("{}", res);
}

fn p1(items: &str, ranges: &Vec<Range<usize>>) -> usize {
    items
        .split('\n')
        .filter_map(|line| {
            if let Ok(i) = usize::from_str_radix(line, 10) {
                Some(i)
            } else {
                None
            }
        })
        .filter(|i| ranges.iter().filter(|range| range.contains(i)).count() > 0)
        .count()
}

fn check_item(item: usize, ranges: &Vec<Range<usize>>) -> bool {
    ranges.iter().filter(|range| range.contains(&item)).count() > 0
}
