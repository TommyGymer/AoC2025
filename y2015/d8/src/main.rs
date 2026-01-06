use std::fs;

use regex::Regex;

fn part_1(lines: Vec<&str>) -> usize {
    let total_string_code_length: usize = lines.iter().map(|line| line.len()).sum();

    let re = Regex::new("(\\\\x[\\da-f][\\da-f]|\\\\.)").unwrap();

    let diff: usize = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            println!("line {}: {}", i, line);
            re.captures_iter(line)
                .map(|special_chars| {
                    println!("line {}: {:?}", i, special_chars);
                    special_chars.get_match().len() - 1
                })
                .sum::<usize>()
                + 2
        })
        .sum();

    println!(
        "{} - {}",
        total_string_code_length,
        (total_string_code_length - diff)
    );

    diff
}

fn part_2(lines: Vec<&str>) -> usize {
    let increase: usize = lines
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter(|&c| c == '\"' || c == '\\')
                .count()
        })
        .sum();

    lines.len() * 2 + increase
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    println!("{}", part_1(lines.to_owned()));
    println!("{}", part_2(lines));
}

#[cfg(test)]
mod tests_part_1 {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(part_1(vec!["\"\""]), 2)
    }

    #[test]
    fn test_abc() {
        assert_eq!(part_1(vec!["\"abc\""]), 2)
    }

    #[test]
    fn test_aaa() {
        assert_eq!(part_1(vec!["\"aaa\\\"aaa\""]), 3)
    }

    #[test]
    fn test_x27() {
        assert_eq!(part_1(vec!["\"\\x27\""]), 5)
    }

    #[test]
    fn test_x27x27() {
        assert_eq!(part_1(vec!["\"\\x27\\x27\""]), 10 - 2)
    }

    #[test]
    fn test_xaa() {
        assert_eq!(part_1(vec!["\"\\xaa\""]), 5)
    }
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(part_2(vec!["\"\""]), 4)
    }

    #[test]
    fn test_abc() {
        assert_eq!(part_2(vec!["\"abc\""]), 4)
    }

    #[test]
    fn test_aaa() {
        assert_eq!(part_2(vec!["\"aaa\\\"aaa\""]), 6)
    }

    #[test]
    fn test_x27() {
        assert_eq!(part_2(vec!["\"\\x27\""]), 5)
    }

    #[test]
    fn test_x27x27() {
        assert_eq!(part_2(vec!["\"\\x27\\x27\""]), 6)
    }

    #[test]
    fn test_xaa() {
        assert_eq!(part_2(vec!["\"\\xaa\""]), 5)
    }
}
