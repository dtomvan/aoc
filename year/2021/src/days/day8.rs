use aoc_common::{
    lines,
    result::{done, AocResult, SSum},
};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = lines!("../../inputs/day-8.txt")
        .filter_map(|x| x.split(" | ").collect_tuple())
        .collect_vec();

    let part_1 = input
        .iter()
        .map(|(_, x)| {
            x.split_whitespace()
                .map(|x| x.len())
                .filter(|&x| x == 2 || x == 4 || x == 3 || x == 7)
                .count()
        })
        .ssum();

    let mut part_2 = 0usize;
    for (inp, outp) in input {
        let inp = inp.split_whitespace().collect_vec();
        let one = inp.iter().find(|x| x.len() == 2).unwrap();
        let four = inp.iter().find(|x| x.len() == 4).unwrap();

        outp.split_whitespace().fold(1000, |acc, x| {
            let num = match (x.len(), common_with(x, one), common_with(x, four)) {
                (2, _, _) => 1,
                (5, _, 2) => 2,
                (5, 2, _) => 3,
                (4, _, _) => 4,
                (5, 1, _) => 5,
                (6, 1, _) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, _, 4) => 9,
                (6, _, 3) => 0,
                _ => panic!("Unexpected 7-seg digit"),
            };
            part_2 += num * acc;
            acc / 10
        });
    }

    done(part_1, part_2)
}

fn common_with(input: &str, str: &str) -> usize {
    input.chars().filter(|x| str.contains(*x)).count()
}
