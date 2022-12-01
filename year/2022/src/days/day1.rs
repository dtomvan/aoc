use aoc_common::result::{done_second, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let mut input = include_str!("../../inputs/day-1.txt")
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .sum::<usize>()
        })
        .sorted_unstable()
        .rev();
    // Part 2
    done_second(input.next().unwrap(), &mut |a| {
        a + input.next_chunk::<2>().unwrap().into_iter().sum::<usize>()
    })
}
