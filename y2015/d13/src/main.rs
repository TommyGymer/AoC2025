use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    i64,
};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RelationType {
    Good,
    Bad,
}

impl<'a, T> From<T> for RelationType
where
    &'a str: From<T>,
{
    fn from(value: T) -> Self {
        match value.into() {
            "gain" => Self::Good,
            "lose" => Self::Bad,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Relation {
    t: RelationType,
    amount: u32,
}

impl Into<i64> for &Relation {
    fn into(self) -> i64 {
        match self.t {
            RelationType::Good => self.amount as i64,
            RelationType::Bad => -(self.amount as i64),
        }
    }
}

#[derive(Debug, Eq)]
struct TwoPeople<'a> {
    a: &'a str,
    b: &'a str,
}

impl<'a> PartialEq for TwoPeople<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b || self.a == other.b && self.b == other.a
    }
}

impl<'a> Hash for TwoPeople<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.a < self.b {
            self.a.hash(state);
            self.b.hash(state);
        } else {
            self.b.hash(state);
            self.a.hash(state);
        }
    }
}

fn solve(
    people: &Vec<&str>,
    selected: &Vec<bool>,
    relations: &HashMap<TwoPeople, i64>,
    prev_person: Option<&str>,
    first_person: Option<&str>,
    depth: usize,
    debug: &Vec<&str>,
) -> i64 {
    println!("{:?} {:?}", first_person, prev_person);
    assert_eq!(people.len(), selected.len());
    let res = match (first_person, prev_person) {
        (None, _) => (0..1)
            .into_iter()
            .map(|i| {
                println!("here {}@{}", i, depth);
                let mut new_selected = selected.to_owned();
                new_selected[i] = true;
                let person = Some(people[i]);

                let mut new_debug = debug.clone();
                new_debug.push(people[i]);

                solve(
                    people,
                    &new_selected,
                    relations,
                    prev_person,
                    person,
                    depth + 1,
                    &new_debug,
                )
            })
            .max()
            .unwrap(),
        (Some(first), Some(prev)) => (0..people.len())
            .into_iter()
            .filter(|i| selected[*i] == false)
            .map(|i| {
                println!("there {}@{}", i, depth);
                let mut new_selected = selected.to_owned();
                new_selected[i] = true;
                let person = Some(people[i]);

                let r = relations
                    .get(&TwoPeople {
                        a: prev,
                        b: people[i],
                    })
                    .unwrap();

                let mut new_debug = debug.clone();
                new_debug.push(people[i]);

                solve(
                    people,
                    &new_selected,
                    relations,
                    person,
                    first_person,
                    depth + 1,
                    &new_debug,
                ) + r
            })
            .max()
            .unwrap_or(*relations.get(&TwoPeople { a: prev, b: first }).unwrap()),
        (Some(first), None) => (0..people.len())
            .into_iter()
            .filter(|i| selected[*i] == false)
            .map(|i| {
                println!("yonder {}@{}", i, depth);
                let mut new_selected = selected.to_owned();
                new_selected[i] = true;
                let person = Some(people[i]);

                let r = relations
                    .get(&TwoPeople {
                        a: first,
                        b: people[i],
                    })
                    .unwrap();

                let mut new_debug = debug.clone();
                new_debug.push(people[i]);

                solve(
                    people,
                    &new_selected,
                    relations,
                    person,
                    first_person,
                    depth + 1,
                    &new_debug,
                ) + r
            })
            .max()
            .expect("oh no"),
    };
    println!("{} with {:?}", depth, debug);
    println!("value {}", res);
    res
}

fn main() {
    let re = Regex::new(
        r"([a-zA-Z]+) would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+).",
    )
    .unwrap();

    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let relations_vec: Vec<(TwoPeople, Relation)> = re
        .captures_iter(&input)
        .map(|item| {
            let (_, [a, t, amount, b]) = item.extract();
            (
                TwoPeople { a, b },
                Relation {
                    t: t.into(),
                    amount: u32::from_str_radix(amount, 10).unwrap(),
                },
            )
        })
        .collect();

    let mut relations: HashMap<TwoPeople, i64> = HashMap::with_capacity(relations_vec.len() / 2);

    for (two_people, relation) in relations_vec {
        if let Some(r) = relations.get_mut(&two_people) {
            let v: i64 = (&relation).into();
            *r += v
        } else {
            relations.insert(two_people, (&relation).into());
        }
    }

    let people_a: HashSet<&str> = relations.iter().map(|relation| relation.0.a).collect();
    let people_b: HashSet<&str> = relations.iter().map(|relation| relation.0.b).collect();
    let people = people_a.union(&people_b);

    let num_people = people.clone().count() + 1;
    let mut people_vec: Vec<&str> = people.into_iter().map(|person| *person).collect();

    for person in people_vec.iter() {
        relations.insert(TwoPeople { a: "Me", b: person }, 0);
    }

    people_vec.push("Me");

    let result: i64 = solve(
        &people_vec,
        &vec![false; num_people],
        &relations,
        None,
        None,
        0,
        &vec![],
    );

    println!("{}", result);
}
