use std::{collections::HashSet, fs, str::Chars};

fn generate_location_set(moves: Vec<char>) -> HashSet<(i32, i32)> {
    let mut locations: HashSet<(i32, i32)> = HashSet::new();

    let mut current = (0, 0);
    locations.insert(current);
    for m in moves {
        match m {
            '^' => current.1 = current.1 + 1,
            'v' => current.1 = current.1 - 1,
            '>' => current.0 = current.0 + 1,
            '<' => current.0 = current.0 - 1,
            _ => {}
        }

        locations.insert(current);
    }

    locations
}

fn part_1(moves: Chars) -> usize {
    generate_location_set(moves.collect()).len()
}

fn alternate_moves(moves: Chars) -> (Vec<char>, Vec<char>) {
    (
        moves.to_owned().step_by(2).collect(),
        moves.skip(1).step_by(2).collect(),
    )
}

fn part_2(moves: Chars) -> usize {
    let (a, b) = alternate_moves(moves);

    let santa = generate_location_set(a);
    let robo = generate_location_set(b);

    santa.union(&robo).count()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", part_1(input.chars()));
    println!("{}", part_2(input.chars()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eg1() {
        assert_eq!(part_2("^v".chars()), 3)
    }

    #[test]
    fn eg2() {
        assert_eq!(part_2("^>v<".chars()), 3)
    }

    #[test]
    fn eg3() {
        assert_eq!(part_2("^v^v^v^v^v".chars()), 11)
    }

    #[test]
    fn alternate() {
        assert_eq!(
            alternate_moves("<><><><><><>".chars()),
            (
                "<<<<<<".chars().collect::<Vec<char>>(),
                ">>>>>>".chars().collect::<Vec<char>>()
            )
        )
    }
}
