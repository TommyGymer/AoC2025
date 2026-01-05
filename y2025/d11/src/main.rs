use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let nodes: HashMap<String, Vec<String>> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut sides = line.split(':');
            let name = sides.next().unwrap().to_owned();
            let links: Vec<String> = sides
                .last()
                .unwrap()
                .split(' ')
                .filter(|link| link.len() > 0)
                .map(|link| link.to_owned())
                .collect();
            (name, links.into_iter().collect())
        })
        .collect();

    let path1 = explore("svr", "dac", &nodes, &mut HashMap::new())
        * explore("dac", "fft", &nodes, &mut HashMap::new())
        * explore("fft", "out", &nodes, &mut HashMap::new());

    let path2 = explore("svr", "fft", &nodes, &mut HashMap::new())
        * explore("fft", "dac", &nodes, &mut HashMap::new())
        * explore("dac", "out", &nodes, &mut HashMap::new());

    println!("{}", path1 + path2);
}

fn explore(
    node: &str,
    end_node: &str,
    nodes: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(res) = cache.get(node) {
        *res
    } else {
        let res = if node == end_node {
            1
        } else {
            match nodes.get(node) {
                Some(next_vec) => next_vec
                    .iter()
                    .map(|next| explore(next, end_node, nodes, cache))
                    .sum(),
                None => 0,
            }
        };
        cache.insert(node.to_owned(), res);
        res
    }
}
