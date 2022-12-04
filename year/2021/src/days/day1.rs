use aoc_common::{
    parse,
    result::{done, AocResult},
};

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-1.txt")
        .lines()
        .filter_map(parse!())
        .collect();

    done(solution(&input, 2), solution(&input, 4))
}

fn solution(i: &Vec<usize>, window: usize) -> usize {
    i.windows(window).filter(|x| x.first() < x.last()).count()
}
