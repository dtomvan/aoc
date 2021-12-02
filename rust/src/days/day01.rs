pub fn main() -> anyhow::Result<(usize, usize)> {
    // Part 1
    let input = include_str!("../../../inputs/day-1.txt");
    let numbers = input.lines().filter_map(|x| x.parse::<usize>().ok());
    let part_1 = solution(&numbers);

    // Part 2
    let part_2 = solution(
        &numbers
            .collect::<Vec<_>>()
            .windows(3)
            .map(|x| x.iter().sum()),
    );

    Ok((part_1, part_2))
}

fn solution<I: Iterator<Item = usize> + Clone>(iter: &I) -> usize {
    iter.clone().zip(iter.clone().skip(1))
        .filter(|x| x.0 < x.1)
        .count()
}
