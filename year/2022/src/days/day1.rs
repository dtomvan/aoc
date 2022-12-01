use aoc_common::result::{done, AocResult};
use itertools::Itertools;
pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-1.txt");
    let mut i_1: Vec<usize> = input
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .sum::<usize>()
        })
        .collect_vec();
    i_1.sort_unstable();
    // Part 2
    done(
        i_1[i_1.len() - 1],
        i_1[i_1.len() - 3..=i_1.len() - 1].into_iter().sum::<usize>(),
    )
}
