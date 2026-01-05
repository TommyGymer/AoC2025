use md5::compute;
use std::fs;

fn find_hash_conflict(input: &str) -> usize {
    let mut i = 1;
    loop {
        let hash = compute(format!("{}{}", input, i));

        if hash[0] & 0xff == 0 && hash[1] & 0xff == 0 && hash[2] & 0xff == 0 {
            return i;
        }

        i += 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", find_hash_conflict(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eg1() {
        assert_eq!(find_hash_conflict("abcdef"), 609043)
    }

    #[test]
    fn eg2() {
        assert_eq!(find_hash_conflict("pqrstuv"), 1048970)
    }
}
