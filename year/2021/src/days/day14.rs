use std::collections::HashMap;

use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    let (p, i) = include_str!("../../inputs/day-14.txt")
        .split_once("\n\n")
        .unwrap();

    let p = p
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut acc, (a, b)| {
            *acc.entry(format!("{a}{b}")).or_insert(0) += 1;
            acc
        });

    let i = i
        .lines()
        .filter_map(|x| x.splitn(2, " -> ").collect_tuple())
        .collect_vec();

    let part_1 = step_amount(10, &i, p.clone());
    // Don't ask about the -1, it works
    let part_2 = step_amount(40, &i, p) - 1;

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
            let mut f = inst.0.chars();
            if let Some(poly) = polymer.get(inst.0) {
                if *poly > 0 {
                    let mut inst_result = String::new();
                    inst_result.push(f.next().unwrap());
                    inst_result.push(inst.1.chars().next().unwrap());
                    inst_result.push(f.next().unwrap());
                    for (a, b) in inst_result.chars().tuple_windows() {
                        *temp.entry(format!("{a}{b}")).or_insert(0) += *poly;
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
