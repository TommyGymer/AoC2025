use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let containers: Vec<(usize, u16)> = input
        .split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| u16::from_str_radix(line, 10).unwrap())
        .enumerate()
        .collect();

    let mut counts: HashMap<u16, u16> = HashMap::new();

    for i in 0..2usize.pow(containers.len() as u32) {
        let mut num_of_containers: u16 = 0;
        let size: u32 = containers
            .iter()
            .map(|(container_i, container)| {
                if i & 2usize.pow(*container_i as u32) != 0 {
                    num_of_containers += 1;
                    *container as u32
                } else {
                    0u32
                }
            })
            .sum();

        if size == 150 {
            match counts.get_mut(&num_of_containers) {
                Some(x) => *x += 1u16,
                None => {
                    counts.insert(num_of_containers, 1);
                }
            }
        }
    }

    println!("{:?}", counts);
}
