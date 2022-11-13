use aoc_common::result::{AocResult, done};

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-1.txt");
    let numbers: Vec<_> = input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let part_1 = solution(&numbers.windows(2));

    // Part 2
    let part_2 = solution(&numbers.windows(4));

    done(part_1, part_2)
}

fn solution<'a, I: Iterator<Item = &'a [usize]> + Clone>(iter: &I) -> usize {
    iter.clone().filter(|x| x.first() < x.last()).count()
}
