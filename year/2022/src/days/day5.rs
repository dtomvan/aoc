use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let (graph, inst) = include_str!("../../inputs/day-5.txt")
        .split_once("\n\n")
        .res()?;

    let inst = inst
        .split_whitespace()
        .filter_map(parse!())
        .array_chunks()
        .collect_vec();

    let data = graph
        .lines()
        .map(|x| x.chars().skip(1).step_by(4).collect_vec())
        .filter(|x| !x.is_empty())
        .collect_vec()
        .transpose()
        .into_iter()
        .map(|x| x.into_iter().filter(|x| x.is_alphabetic()).rev().collect())
        .collect_vec();

    done(s(inst.clone(), data.clone(), true), s(inst, data, false))
}

fn s(inst: Vec<[usize; 3]>, mut m: Vec<Vec<char>>, part: bool) -> String {
    for [amount, stack, target] in inst {
        let s = m.get_mut(stack - 1).unwrap();
        let to_push = s.split_off(s.len().saturating_sub(amount));
        let target = m.get_mut(target - 1).unwrap();
        if part {
            target.extend(to_push.into_iter().rev());
        } else {
            target.extend(to_push);
        }
    }
    m.iter().filter_map(|x| x.last()).collect()
}
