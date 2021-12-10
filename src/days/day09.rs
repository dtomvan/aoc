use itertools::Itertools;

pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../inputs/day-9.txt");
    // NOTICE: You index like [y][x], not [x][y]
    let map = input
        .lines()
        .map(|x| x.chars().filter_map(|y| y.to_digit(10)).collect_vec())
        .collect_vec();
    let mut low_indices = Vec::new();
    let mut part_1 = 0;
    for (i, x) in map.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            let (up, down, left, right) = get_directions(&map, i, j);

            if y < up && y < down && y < left && y < right {
                low_indices.push((i, j));
                part_1 += y + 1;
            }
        }
    }
    let mut basin_sizes = Vec::new();
    for (i, j) in low_indices.iter() {
        basin_sizes.push(acc_basin_size(&mut vec![], 0, &map, *i, *j));
    }
    basin_sizes.sort();
    let part_2 = basin_sizes.into_iter().rev().take(3).fold(1, |acc, x| acc * x);

    Ok((part_1 as usize, part_2 as usize))
}

fn acc_basin_size(mut scanned: &mut Vec<(usize, usize)>, acc: u32, map: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let num = map.get(i).and_then(|x| x.get(j));
    if num.is_none() { return 0; }

    let (up, down, left, right) = get_directions(&map, i, j);
    [up, down, left, right].into_iter().enumerate().fold(acc, |acc, (k, x)| {
        let (i, j) = match k {
            0 => (i.overflowing_sub(1).0, j),
            1 => (i.overflowing_add(1).0, j),
            2 => (i, j.overflowing_sub(1).0),
            3 => (i, j.overflowing_add(1).0),
            _ => unreachable!(),
        };
        if x < &9 && !scanned.contains(&(i, j)) {
            scanned.push((i, j));
            acc_basin_size(&mut scanned, acc + 1, &map, i, j)
        } else {
            acc
        }
    })
}

fn get_directions(map: &Vec<Vec<u32>>, i: usize, j: usize) -> (&u32, &u32, &u32, &u32) {
    let up = if i == 0 { &10 } else { &map[i - 1][j] };
    let down = map.get(i + 1).and_then(|x| x.get(j)).unwrap_or(&10);
    let left = if j == 0 { &10 } else { &map[i][j - 1] };
    let right = map[i].get(j + 1).unwrap_or(&10);

    (up, down, left, right)
}
