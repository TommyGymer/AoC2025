use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u64,
    time: u64,
    rest: u64,
}

fn fastest<'a>(reindeer: &'a Vec<Reindeer>, race_length: u64) -> &'a str {
    let fastest = reindeer
        .iter()
        .map(|r| {
            let full = race_length / (r.time + r.rest);
            let part = race_length % (r.time + r.rest);

            (
                &r.name,
                (r.speed * (r.time * full)) + (part.min(r.time) * r.speed),
            )
        })
        .reduce(|best, next| if best.1 > next.1 { best } else { next });
    fastest.unwrap().0
}

fn main() {
    let re = Regex::new(
        r"([a-zA-Z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let reindeer: Vec<Reindeer> = re
        .captures_iter(&input)
        .map(|item| {
            let (_, [name, speed, time, rest]) = item.extract();
            Reindeer {
                name: String::from(name),
                speed: u64::from_str_radix(speed, 10).unwrap(),
                time: u64::from_str_radix(time, 10).unwrap(),
                rest: u64::from_str_radix(rest, 10).unwrap(),
            }
        })
        .collect();

    println!("{:?}", reindeer);

    let mut score_card: HashMap<&str, u64> = HashMap::new();

    for i in 0..2503 {
        let in_first = fastest(&reindeer, i);
        if let Some(scorer) = score_card.get_mut(in_first) {
            *scorer += 1;
        } else {
            score_card.insert(in_first, 1);
        }
    }

    println!("{:?}", score_card);
}
