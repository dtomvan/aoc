use aoc_common::result::{done, AocResult};
use std::collections::{HashMap, VecDeque};

pub fn main() -> AocResult {
    let (graph, inst) = include_str!("../../inputs/day-5.txt")
        .split_once("\n\n")
        .unwrap();

    let mut p1 = graph.lines().fold(HashMap::new(), |mut acc, x| {
        x.chars().skip(1).step_by(4).enumerate().for_each(|(x, c)| {
            if c.is_alphabetic() {
                acc.entry(x + 1).or_insert_with(VecDeque::new).push_front(c);
            }
        });
        acc
    });
    let mut p2 = p1.clone();

    for [amount, stack, target] in inst
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .array_chunks()
    {
        // Part 1
        (0..amount).for_each(|_| {
            if let Some(c) = p1.get_mut(&stack).unwrap().pop_back() {
                p1.get_mut(&target).unwrap().push_back(c);
            }
        });

        // Part 2
        let stack = p2.get_mut(&stack).unwrap();
        let to_push = stack.split_off(stack.len().saturating_sub(amount));
        p2.get_mut(&target).unwrap().extend(to_push);
    }

    let w = p1.keys().count();
    done(s(&p1, w), s(&p2, w))
}

fn s(m: &HashMap<usize, VecDeque<char>>, w: usize) -> String {
    (1..=w)
        .into_iter()
        .map(|s| *m.get(&s).unwrap().back().unwrap_or(&' '))
        .collect()
}
