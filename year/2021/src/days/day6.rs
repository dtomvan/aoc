use aoc_common::result::{AocResult, done};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn main() -> AocResult {
    let input: Vec<usize> = include_str!("../../inputs/day-6.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let mut fish: VecDeque<_> = [0; 9].into();

    for fish_in in input {
        fish[fish_in] += 1;
    }

    for _ in 0..80 {
        cycle(&mut fish);
    }

    let part_1: usize = fish.iter().sum();

    for _ in 0..176 {
        cycle(&mut fish);
    }

    let part_2: usize = fish.iter().sum();

    done(part_1, part_2)
}

fn cycle(fish: &mut VecDeque<usize>) {
    let birthing = fish.pop_front().unwrap();
    fish[6] += birthing;
    fish.push_back(birthing);
}
