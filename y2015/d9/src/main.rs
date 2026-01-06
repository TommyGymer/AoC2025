use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn path_value(path: &Vec<&str>, edges: &HashMap<(&str, &str), usize>) -> Option<usize> {
    let mut prev: &str = path.first().unwrap();
    let mut total: usize = 0;

    for node in path.iter().skip(1) {
        match (edges.get(&(prev, node)), edges.get(&(node, prev))) {
            (Some(value), None) => total += value,
            (None, Some(value)) => total += value,
            (None, None) => return None,
            (Some(_), Some(_)) => unimplemented!(),
        }
        prev = node
    }

    println!("{:?}", path);

    Some(total)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    let edges: HashMap<(&str, &str), usize> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            let nodes: Vec<&str> = parts.get(0).unwrap().split(" to ").collect();

            (
                (*nodes.get(0).unwrap(), *nodes.get(1).unwrap()),
                usize::from_str_radix(parts.get(1).unwrap(), 10).unwrap(),
            )
        })
        .collect();

    println!("{:?}", edges);

    let mut nodes: HashSet<&str> = HashSet::new();
    for ((a, b), _) in &edges {
        nodes.insert(a);
        nodes.insert(b);
    }

    println!("{:?}", nodes);

    let nodes_len = nodes.len();
    let res = nodes
        .into_iter()
        .permutations(nodes_len)
        .map(|path| {
            println!("{:?}", path);
            path
        })
        .filter_map(|path| path_value(&path, &edges))
        .max()
        .unwrap();

    println!("{}", res);
}
