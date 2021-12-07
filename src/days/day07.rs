use itertools::Itertools;

pub fn main() -> anyhow::Result<(usize, usize)> {
    let mut input = include_str!("../../inputs/day-7.txt")
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<isize>().ok())
        .collect_vec();

    input.sort_unstable();

    let half_len = input.len() / 2;
    let mean = ((input[half_len] + input[(half_len) + input.len() % 2]) / 2) as isize;
    let total_fuel = input.iter().fold(0, |acc, x| acc + (x - mean).abs());

    let part_2 = (input[0]..input[input.len() - 1]).fold(0, |acc, x| {
        let fuel = input.iter().fold(0, |acc, y| acc + sum((x - y).abs()));

        if fuel < acc || acc == 0 {
            fuel
        } else {
            acc
        }
    });

    Ok((total_fuel as usize, part_2 as usize))
}

fn sum(n: isize) -> isize {
    (n * (n + 1)) / 2
}
