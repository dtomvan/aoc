pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../inputs/day-3.txt");

    let numbers: Vec<_> = input
        .lines()
        .filter_map(|x| usize::from_str_radix(x, 2).ok())
        .collect();
    let code_length = input.lines().next().unwrap().len();

    // Part 1
    let most_common = (0..code_length)
        .map(|i| find_most_common(&numbers, i) << i)
        .sum::<usize>();

    // 0xFF is just 12 bits
    let least_common = !most_common & 0xFFF;

    // Part 2

    Ok((
        most_common * least_common,
        part_2(code_length, &numbers, false) * part_2(code_length, &numbers, true),
    ))
}

fn find_most_common(nums: &[usize], bit: usize) -> usize {
    let mut sum = 0;
    for x in nums.iter() {
        if (x >> bit) & 1 == 1 {
            sum += 1;
        } else {
            sum -= 1;
        }
    }
    (sum >= 0) as usize
}

fn part_2(length: usize, numbers: &[usize], invert: bool) -> usize {
    let mut numbers = numbers.to_vec();

    for i in (0..length).rev() {
        let common = find_most_common(&numbers, i);

        numbers.retain(|x| ((x >> i) & 1 == common) != invert);
        if numbers.len() == 1 {
            break;
        }
    }
    numbers[0]
}
