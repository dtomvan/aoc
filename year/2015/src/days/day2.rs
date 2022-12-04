use aoc_common::{
    parse,
    result::{done, AocResult},
};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-2.txt");
    let input: Vec<(_, _, _)> = input
        .lines()
        .map(|line| {
            line.splitn(3, 'x')
                .filter_map(parse!(usize))
                .collect_tuple()
                .expect("Incorrect puzzle input")
        })
        .collect();
    let part1 = input.iter().fold(0, |n, (l, w, h)| {
        let b_t = l * w;
        let l_r = w * h;
        let f_b = h * l;
        let slack = b_t.min(l_r).min(f_b);
        n + (2 * b_t) + (2 * l_r) + (2 * f_b) + slack
    });
    // Part 2
    let part2 = input.iter().fold(0, |n, (l, w, h)| {
        let mut s = [l, w, h];
        s.sort_unstable();
        n + (l * w * h) + (s[0] * 2) + (s[1] * 2)
    });
    done(part1, part2)
}
