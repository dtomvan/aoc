pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../inputs/day-10.txt");

    let mut part_1 = 0;
    let mut part_2 = Vec::new();

    'outer: for line in input.lines() {
        let mut brackets = Vec::with_capacity(line.len());
        for x in line.chars() {
            if let ')' | ']' | '}' | '>' = x {
                if let Some(y) = brackets.pop() {
                    match (y, x) {
                        ('(', ')') => continue,
                        ('[', ']') => continue,
                        ('{', '}') => continue,
                        ('<', '>') => continue,
                        (_, ')') => part_1 += 3,
                        (_, ']') => part_1 += 57,
                        (_, '}') => part_1 += 1197,
                        (_, '>') => part_1 += 25137,
                        _ => unreachable!(),
                    }
                    continue 'outer;
                }
            } else {
                brackets.push(x);
            }
        }

        let mut total_points = 0;
        for bracket in brackets.into_iter().rev() {
            let score = match bracket {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };

            total_points = (total_points * 5) + score;
        }
        part_2.push(total_points);
    }
    part_2.sort_unstable();

    Ok((part_1, part_2[part_2.len() / 2]))
}
