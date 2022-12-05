use std::collections::HashMap;

use aoc_common::prelude::*;
use itertools::Itertools;

pub fn main() -> AocResult {
    let input: HashMap<_, _> = include_str!("../../inputs/day-12.txt")
        .lines()
        .filter_map(|x| x.split('-').collect_tuple::<(_, _)>())
        .flat_map(|x| [(x.0, x.1), (x.1, x.0)])
        .into_group_map_by(|x| x.0)
        .into_iter()
        .map(|x| (x.0, x.1.into_iter().map(|y| y.1).collect_vec()))
        .collect();

    let src = input.iter().find(|x| x.0 == &"start").unwrap();

    let day_1 = solve(&input, src.0, &mut vec![], true);
    let day_2 = solve(&input, src.0, &mut vec![], false);

    done(day_1, day_2)
}

fn solve<'a>(
    input: &'a HashMap<&str, Vec<&str>>,
    src: &'a str,
    cur_path: &mut Vec<&'a str>,
    mut seen: bool,
) -> usize {
    if src == "end" {
        return 1;
    }
    if cur_path.contains(&src) && src.chars().all(|x| x.is_lowercase()) {
        if src == "start" || seen {
            return 0;
        }
        seen = true;
    }
    cur_path.push(src);
    let result = input[src]
        .iter()
        .map(|x| solve(input, x, cur_path, seen))
        .sum();
    cur_path.pop();
    result
}
