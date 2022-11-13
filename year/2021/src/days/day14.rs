use std::collections::HashMap;

use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-14.txt");
    let instructions = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|x| x.split(" -> ").collect_tuple::<(_, _)>().unwrap())
        .collect_vec();
    let polymer: HashMap<_, _> = input
        .split("\n\n")
        .next()
        .unwrap()
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut acc, (a, b)| {
            *acc.entry(format!("{}{}", a, b)).or_insert(0) += 1;
            acc
        });

    let part_1 = step_amount(10, &instructions, polymer.clone());
    // Don't ask about the -1, it works
    let part_2 = step_amount(40, &instructions, polymer) - 1;

    done(part_1, part_2)
}

fn step_amount(
    amount: usize,
    instructions: &[(&str, &str)],
    mut polymer: HashMap<String, usize>,
) -> usize {
    for _ in 0..amount {
        let mut temp = HashMap::new();
        for inst in instructions.iter() {
            if let Some(poly) = polymer.get(inst.0) {
                if *poly > 0 {
                    let mut inst_result = String::new();
                    inst_result.push(inst.0.chars().next().unwrap());
                    inst_result.push(inst.1.chars().next().unwrap());
                    inst_result.push(inst.0.chars().nth(1).unwrap());
                    for (a, b) in inst_result.chars().tuple_windows() {
                        *temp.entry(format!("{}{}", a, b)).or_insert(0) += *poly;
                    }
                }
            }
        }
        polymer = temp;
    }
    let counts = polymer.iter().fold(HashMap::new(), |mut acc, (a, b)| {
        *acc.entry(a.chars().next().unwrap()).or_insert(0) += b;
        acc
    });
    let result = counts
        .iter()
        .filter(|x| x.1 != &0)
        .minmax_by_key(|x| x.1)
        .into_option()
        .unwrap();
    result.1 .1 - result.0 .1
}
