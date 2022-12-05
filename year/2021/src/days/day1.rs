use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-1.txt")
        .lines()
        .filter_map(parse!())
        .collect::<Vec<_>>();

    done(solution(&input, 2), solution(&input, 4))
}

fn solution(i: &[usize], window: usize) -> usize {
    i.windows(window).filter(|x| x.first() < x.last()).count()
}
