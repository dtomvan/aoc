use aoc_common::{result::{done, AocResult}, parse};
use itertools::Itertools;

pub fn main() -> AocResult {
    let mut input = include_str!("../../inputs/day-7.txt")
        .trim()
        .split(',')
        .filter_map(parse!(isize))
        .collect_vec();

    input.sort_unstable();

    let half_len = input.len() / 2;
    let median = (input[half_len] + input[(half_len) + input.len() % 2]) / 2;
    let part_1 = input.iter().fold(0, |acc, x| acc + (x - median).abs());
    let mean = input.iter().sum::<isize>() / input.len() as isize;

    let part_2 = input.iter().fold(0, |acc, x| acc + sum((x - mean).abs()));

    done(part_1 as usize, part_2 as usize)
}

fn sum(n: isize) -> isize {
    (n * (n + 1)) / 2
}
