use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-14.txt");

    done(part_1(input)?, part_2(input)?)
}

fn part_1(input: &str) -> Result<usize> {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem = HashMap::new();

    for line in input.lines() {
        let (key, value) = line.split_once(" = ").unwrap();
        if line.as_bytes()[1] == b'a' {
            let (a, o) = read_mask1(value);
            and_mask = a;
            or_mask = o;
        } else {
            *mem.entry(key[4..key.len() - 1].parse::<usize>().unwrap())
                .or_insert(0usize) = value.parse::<usize>()? & and_mask | or_mask;
        }
    }
    Ok(mem.values().sum())
}

fn part_2(input: &str) -> Result<usize> {
    let mut permutations = vec![];
    let mut or_mask = 0;
    let mut mem = HashMap::new();

    for line in input.lines() {
        let (key, value) = line.split_once(" = ").unwrap();
        if line.as_bytes()[1] == b'a' {
            let (o, p) = read_mask2(value);
            or_mask = o;
            permutations = p;
        } else {
            let addr = key[4..key.len() - 1].parse::<usize>().unwrap() | or_mask;
            let value = value.parse()?;
            permutations.iter().for_each(|(a, o)| {
                mem.insert(addr & a | o, value);
            });
        }
    }
    Ok(mem.values().sum())
}

fn read_mask1(inp: &str) -> (usize, usize) {
    let mut and_mask = 0xFFFFFFFFF;
    let mut or_mask = 0;

    for (i, c) in inp.chars().rev().enumerate() {
        match c {
            '1' => or_mask |= 1 << i,
            '0' => and_mask &= !(1 << i),
            _ => continue,
        }
    }
    (and_mask, or_mask)
}

fn read_mask2(inp: &str) -> (usize, Vec<(usize, usize)>) {
    let mut permutations = vec![(0xFFFFFFFFF, 0usize)];
    let mut or_mask = 0;

    for (i, c) in inp.chars().rev().enumerate() {
        match c {
            '1' => or_mask |= 1 << i,
            'X' => {
                permutations = permutations
                    .into_iter()
                    .flat_map(|(a, o)| vec![(a, o | 1 << i), (a ^ (1 << i), o)])
                    .collect_vec()
            }
            _ => continue,
        }
    }
    (or_mask, permutations)
}
