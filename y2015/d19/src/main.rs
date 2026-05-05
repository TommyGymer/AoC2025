use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Rule {
    from: String,
    to: String,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut medcine: String = String::from("");
    let mut rules: Vec<Rule> = vec![];

    let mut found_empty = false;
    for line in lines {
        if found_empty {
            medcine = line.trim().to_owned();
        } else if line.len() == 0 {
            found_empty = true;
        } else {
            let mut bits = line.split(" => ");
            rules.push(Rule {
                from: bits.next().unwrap().to_owned(),
                to: bits.next().unwrap().to_owned(),
            })
        }
    }

    println!("Rules: {:#?}", rules);
    println!("Medicine: {}", medcine);

    let mut new: HashSet<String> = HashSet::new();

    for (i, c) in medcine.char_indices() {
        for rule in &rules {
            if rule.from.len() == 1 {
                if rule.from.chars().next().unwrap() == c {
                    let mut mut_medicine = medcine.clone();
                    mut_medicine.replace_range(i..i + 1, &rule.to);
                    new.insert(mut_medicine);
                }
            }
        }
    }

    let mut prev = None;
    for (i, c) in medcine.char_indices() {
        if let Some(p) = prev {
            for rule in &rules {
                if rule.from.len() == 2 {
                    let mut rule_from_iter = rule.from.chars();
                    if rule_from_iter.next().unwrap() == p && rule_from_iter.next().unwrap() == c {
                        let mut mut_medicine = medcine.clone();
                        mut_medicine.replace_range(i - 1..i + 1, &rule.to);
                        new.insert(mut_medicine);
                    }
                }
            }
        }
        prev = Some(c);
    }

    println!("{}", new.into_iter().count());
}
