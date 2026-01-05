use std::{fs, ops::Range};

use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let (top, bottom) = (sections.next().unwrap(), sections.next().unwrap());

    let mut largest = 0;
    let mut ranges: Vec<Range<usize>> = top
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
            start..end
        })
        .collect();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged = vec![];
    for range in ranges {
        match (
            get_merge_range(&range.start, &merged),
            get_merge_range(&range.end, &merged),
        ) {
            (None, None) => merged.push(range),
            (Some(a), Some(b)) => {
                if a != b {
                    merged.get_mut(a).unwrap().end = merged.remove(b).end
                }
            }
            (Some(a), None) => merged.get_mut(a).unwrap().end = range.end,
            (None, Some(b)) => merged.get_mut(b).unwrap().start = range.start,
        }
    }

    merged.sort_by(|a, b| a.end.cmp(&b.end));
    let ranges = merged;
    let mut merged = vec![];
    for range in ranges {
        match (
            get_merge_range(&range.start, &merged),
            get_merge_range(&range.end, &merged),
        ) {
            (None, None) => merged.push(range),
            (Some(a), Some(b)) => {
                if a != b {
                    merged.get_mut(a).unwrap().end = merged.remove(b).end
                }
            }
            (Some(a), None) => merged.get_mut(a).unwrap().end = range.end,
            (None, Some(b)) => merged.get_mut(b).unwrap().start = range.start,
        }
    }

    println!("{:?}", merged);

    let res: usize = merged.into_iter().map(|r| r.count()).sum();

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
        .filter(|i| check_item(i, ranges))
        .count()
}

fn check_item(item: &usize, ranges: &Vec<Range<usize>>) -> bool {
    ranges.iter().filter(|range| range.contains(item)).count() > 0
}

fn get_merge_range(item: &usize, ranges: &Vec<Range<usize>>) -> Option<usize> {
    ranges
        .iter()
        .enumerate()
        .filter(|(_, range)| range.contains(item))
        .next()
        .map(|(i, _)| i)
}
