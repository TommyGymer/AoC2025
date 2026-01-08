use std::fs;

use fancy_regex::Regex;

fn check_pass(pass: &str) -> bool {
    if pass.contains(['i', 'o', 'l']) {
        false
    } else {
        let doublet_re = Regex::new("(.)\\1.*(.)\\2").unwrap();
        if !doublet_re.is_match(pass).unwrap() {
            false
        } else {
            let mut prev_char: Option<char> = None;
            let mut seq_len = 1;
            for c in pass.chars() {
                match prev_char {
                    None => prev_char = Some(c),
                    Some(prev) => {
                        if (c as u8).checked_sub(prev as u8) == Some(1) {
                            prev_char = Some(c);
                            seq_len += 1
                        } else {
                            prev_char = Some(c);
                            seq_len = 1;
                        }
                    }
                }
                if seq_len == 3 {
                    return true;
                }
            }
            false
        }
    }
}

fn increment_char(c: &mut u8) -> bool {
    match *c as char {
        'a'..='y' => {
            *c = *c + 1;
            false
        }
        'z' => {
            *c = 'a' as u8;
            true
        }
        _ => unimplemented!(),
    }
}

fn increment_str(input: &mut str) {
    // assumes that the string is ascii
    assert!(input.is_ascii());
    unsafe {
        let iter = input.as_bytes_mut();
        iter.reverse();
        for c in iter.iter_mut() {
            if !increment_char(c) {
                break;
            }
        }
        iter.reverse();
    }
}

fn main() {
    let mut input: String = fs::read_to_string("input.txt").unwrap().trim().to_owned();

    while !check_pass(&input) {
        increment_str(&mut input);
    }

    increment_str(&mut input);

    while !check_pass(&input) {
        increment_str(&mut input);
    }

    println!("{}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        assert!(!check_pass("hijklmmn"))
    }

    #[test]
    fn test1() {
        assert!(!check_pass("abbceffg"))
    }
}
