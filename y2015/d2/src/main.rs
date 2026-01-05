use std::fs;

fn part_1(cuboids: &Vec<Vec<u32>>) -> u32 {
    cuboids
        .iter()
        .map(|cuboid| {
            assert_eq!(cuboid.len(), 3);
            cuboid.get(0).unwrap() * cuboid.get(1).unwrap() * 3
                + cuboid.get(0).unwrap() * cuboid.get(2).unwrap() * 2
                + cuboid.get(1).unwrap() * cuboid.get(2).unwrap() * 2
        })
        .sum()
}

fn part_2(cuboids: &Vec<Vec<u32>>) -> u32 {
    cuboids
        .iter()
        .map(|cuboid| {
            assert_eq!(cuboid.len(), 3);
            cuboid.get(0).unwrap() * 2
                + cuboid.get(1).unwrap() * 2
                + cuboid.get(0).unwrap() * cuboid.get(1).unwrap() * cuboid.get(2).unwrap()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines = input.split('\n');

    let cuboids: Vec<Vec<u32>> = lines
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut dimentions: Vec<u32> = line
                .split('x')
                .map(|d| u32::from_str_radix(d, 10).unwrap())
                .collect();
            dimentions.sort();
            dimentions
        })
        .collect();

    println!("{}", part_1(&cuboids));
    println!("{}", part_2(&cuboids));
}
