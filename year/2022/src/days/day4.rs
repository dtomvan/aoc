use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 2
    let part2 = include_str!("../../inputs/day-4.txt")
        .lines()
        .filter_map(|x| {
            x.split(',')
                .flat_map(|x| {
                    x.split('-')
                        .flat_map(|x| x.parse::<usize>().ok())
                        .collect_tuple()
                })
                .collect_tuple()
        })
        .filter(|((f1, f2), (s1, s2))| (*f1..=*f2).any(|x| (*s1..=*s2).contains(&x)))
        .collect_vec();

    // Part 1
    let part1 = part2
        .iter()
        .filter(|((f1, f2), (s1, s2))| {
            (*f1..=*f2).all(|x| (*s1..=*s2).contains(&x))
                || (*s1..=*s2).all(|x| (*f1..=*f2).contains(&x))
        })
        .count();
    done(part1, part2.len())
}
