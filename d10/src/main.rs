mod nnls;

use ndarray::{Array1, Array2, arr1};
use nnls::nnls;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split(' ');
        Self {
            lights: parts
                .next()
                .unwrap()
                .replace("[", "")
                .replace("]", "")
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
                .collect(),
            buttons: parts
                .to_owned()
                .take_while(|p| p.starts_with('('))
                .map(|button| {
                    button
                        .replace("(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|i| usize::from_str_radix(i, 10).unwrap())
                        .collect()
                })
                .collect(),
            joltages: parts
                .last()
                .unwrap()
                .replace("{", "")
                .replace("}", "")
                .split(',')
                .map(|i| usize::from_str_radix(i, 10).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct QueuedLights {
    depth: usize,
    lights: Vec<bool>,
}

fn fewest_buttons_lights(lights: Vec<bool>, buttons: Vec<Vec<usize>>) -> usize {
    let mut queue: VecDeque<QueuedLights> = VecDeque::new();
    queue.push_back(QueuedLights {
        depth: 0,
        lights: lights,
    });

    loop {
        let item = queue.pop_front().unwrap();
        // NOTE: check if all the lights are off:
        // by going from the required state to the starting all off state
        if item.lights.iter().all(|l| !*l) {
            return item.depth;
        } else {
            let _ = buttons
                .iter()
                .map(|button| {
                    let mut lights = item.lights.to_owned();
                    let _ = button
                        .iter()
                        .map(|i| *lights.get_mut(*i).unwrap() = !lights.get(*i).unwrap())
                        .collect::<()>();
                    queue.push_back(QueuedLights {
                        depth: item.depth + 1,
                        lights,
                    });
                })
                .collect::<()>();
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct QueuedCounters {
    depth: usize,
    counters: Vec<usize>,
}

fn fewest_buttons_counters(joltages: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let mut a: Array2<f64> = Array2::zeros((joltages.len(), buttons.len()));
    let _ = buttons
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.into_iter()
                .map(|col| {
                    a[[*col, j]] = 1f64;
                })
                .collect::<()>()
        })
        .collect::<()>();
    let b: Array1<f64> = arr1(
        joltages
            .iter()
            .map(|i| *i as f64)
            .collect::<Vec<f64>>()
            .as_slice(),
    );

    // get an approximate solution
    let (sol, err) = nnls(a.view(), b.view());
    println!("{:?}", err);
    // println!("{:?}", sol);

    // determine an exact solution
    // println!("{:?}", joltages);
    let joltages_len = joltages.len();
    let combinations = 2usize.pow(sol.len() as u32);
    (0..combinations)
        .map(|i| {
            let mut presses = 0;
            let counters = buttons
                .iter()
                .zip(sol.to_vec())
                .enumerate()
                .map(|(j, (button, s))| {
                    if i & (1 << j) == 0 {
                        let mut counters = vec![0; joltages_len];
                        presses += s.floor() as usize;
                        let _ = button
                            .iter()
                            .map(|b| *counters.get_mut(*b).unwrap() += s.floor() as usize)
                            .collect::<()>();
                        counters
                    } else {
                        let mut counters = vec![0; joltages.len()];
                        presses += s.ceil() as usize;
                        let _ = button
                            .iter()
                            .map(|b| *counters.get_mut(*b).unwrap() += s.ceil() as usize)
                            .collect::<()>();
                        counters
                    }
                })
                .reduce(|a, b| a.iter().zip(b).map(|(a, b)| a + b).collect())
                .expect("there were no buttons");
            (presses, counters)
        })
        .filter(|(_, counters)| {
            // println!("{:?} == {:?}", joltages, counters);
            joltages.eq(counters)
        })
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .expect("there were no matches")
        .0
}

fn main() {
    let input = std::fs::read_to_string("example.txt").expect("unable to read file");

    let machine_line = input.split('\n');
    let machines: Vec<Machine> = machine_line
        .filter(|line| line.len() > 0)
        .map(|line| Machine::from(line))
        .collect();

    let machines_len = machines.len();

    let res: usize = machines
        .into_iter()
        .enumerate()
        .map(|(i, machine)| {
            let res = fewest_buttons_counters(machine.joltages, machine.buttons);
            println!("finished machine {} of {}", i + 1, machines_len);
            res
        })
        .sum();

    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewest_buttons() {
        let res = fewest_buttons_lights(
            vec![false, true, true, true, false, true],
            vec![vec![0, 3, 4], vec![0, 1, 2, 4, 5]],
        );

        assert_eq!(res, 2);
    }
}
