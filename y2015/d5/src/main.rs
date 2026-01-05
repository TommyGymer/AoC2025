use std::fs;

use fancy_regex::Regex;

fn check_nice(string: &str) -> bool {
    let vowel_count = string
        .chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count();

    let disallowed = string.contains("ab")
        || string.contains("cd")
        || string.contains("pq")
        || string.contains("xy");

    let mut prev = ' ';
    let mut has_duplicate = false;
    for char in string.chars() {
        if prev == char {
            has_duplicate = true;
            break;
        }
        prev = char
    }

    vowel_count >= 3 && !disallowed && has_duplicate
}

fn check_extra_nice(string: &str) -> bool {
    let prop1 = Regex::new("(..).*\\1").unwrap();
    let prop2 = Regex::new("(.).\\1").unwrap();

    prop1.is_match(string).unwrap() && prop2.is_match(string).unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    let res = lines.iter().filter(|line| check_extra_nice(line)).count();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eg1() {
        assert!(check_nice("ugknbfddgicrmopn"))
    }

    #[test]
    fn eg2() {
        assert!(check_nice("aaa"))
    }

    #[test]
    fn eg3() {
        assert!(!check_nice("jchzalrnumimnmhp"))
    }

    #[test]
    fn eg4() {
        assert!(!check_nice("haegwjzuvuyypxyu"))
    }

    #[test]
    fn eg5() {
        assert!(!check_nice("dvszwmarrgswjxmb"))
    }

    #[test]
    fn eg6() {
        assert!(check_extra_nice("qjhvhtzxzqqjkmpb"))
    }

    #[test]
    fn eg7() {
        assert!(check_extra_nice("xxyxx"))
    }

    #[test]
    fn eg8() {
        assert!(!check_extra_nice("uurcxstgmygtbstg"))
    }

    #[test]
    fn eg9() {
        assert!(!check_extra_nice("ieodomkazucvgmuy"))
    }
}
