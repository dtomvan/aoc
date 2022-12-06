use aoc_common::prelude::*;

pub fn main() -> AocResult {
    // Part 2
    let part2 = lines!("../../inputs/day-4.txt")
        .filter_map(|x| {
            x.splitn(4, ['-', ','])
                .flat_map(parse!(usize))
                .collect_tuple()
        })
        .filter(|(f1, f2, s1, s2)| !(f2 < s1 || s2 < f1))
        .collect_vec();

    // Part 1
    let part1 = part2
        .iter()
        .filter(|(f1, f2, s1, s2)| (f1 <= s1 && f2 >= s2) || (s1 <= f1 && s2 >= f2));

    done(part1.count(), part2.len())
}
