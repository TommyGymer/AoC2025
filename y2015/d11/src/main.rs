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
                println!("{} {} {:?}", c, seq_len, prev_char);
                match prev_char {
                    None => prev_char = Some(c),
                    Some(prev) => {
                        if c as u8 - prev as u8 == 1 {
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

fn increment_char(c: &mut char) -> bool {
    match c {
        'a'..='y' => {
            *c = (*c as u8 + 1) as char;
            false
        }
        'z' => {
            *c = 'a';
            true
        }
        _ => unimplemented!(),
    }
}

fn increment_str(input: &mut str) {
    for i in 0..input.len() {}
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap().trim().to_owned();
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
