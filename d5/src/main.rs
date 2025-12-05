use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let (top, bottom) = (sections.next().unwrap(), sections.next().unwrap());

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
            Range {
                start: parts.next().unwrap(),
                end: parts.next().unwrap() + 1,
            }
        })
        .collect();

    let res: usize = bottom
        .split('\n')
        .filter_map(|line| {
            if let Ok(i) = usize::from_str_radix(line, 10) {
                Some(i)
            } else {
                None
            }
        })
        .filter(|i| ranges.iter().filter(|range| range.contains(i)).count() > 0)
        .count();

    println!("{}", res);
}
