use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-8.txt");

    let parsed_input = input
        .lines()
        .map(|x| x.split(" | ").collect_tuple::<(_, _)>().unwrap())
        .collect_vec();

    let part_1: usize = parsed_input
        .iter()
        .map(|x| {
            x.1.split_whitespace()
                .map(|x| x.len())
                // 1 4 7 8
                .filter(|&x| x == 2 || x == 4 || x == 3 || x == 7)
                .count()
        })
        .sum();

    let mut sum = 0usize;
    for (inp, outp) in parsed_input {
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
            sum += num * acc;
            acc / 10
        });
    }

    done(part_1, sum)
}

fn common_with(input: &str, str: &str) -> usize {
    input.chars().filter(|x| str.contains(*x)).count()
}
