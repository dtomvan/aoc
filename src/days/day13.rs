use itertools::Itertools;

pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../inputs/day-13.txt");
    let mut dots = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (x, y): (usize, usize) = x
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y)
        })
        .collect_vec();
    let fold_inst = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|x| {
            let (dir, amount) = x[11..].split('=').collect_tuple::<(_, _)>().unwrap();
            (dir, amount.parse::<usize>().unwrap())
        })
        .collect_vec();

    let (dir, amount) = fold_inst[0];
    let mut day_1 = fold(dots.clone(), dir, amount);
    day_1.sort_unstable();
    day_1.dedup();
    for fd in fold_inst {
        dots = fold(dots, fd.0, fd.1);
    }
    dots.sort_unstable();
    dots.dedup();
    print_dots(dots);

    Ok((day_1.len(), 0))
}

fn fold(mut input: Vec<(usize, usize)>, dir: &str, amount: usize) -> Vec<(usize, usize)> {
    match dir {
        "x" => {
            for (x, _) in input.iter_mut() {
                if *x > amount {
                    *x = amount - (*x - amount);
                }
            }
        }
        "y" => {
            for (_, y) in input.iter_mut() {
                if *y > amount {
                    *y = amount - (*y - amount);
                }
            }
        }
        _ => unimplemented!(),
    }
    input
}

#[cfg(not(debug_assertions))]
fn print_dots(dots: Vec<(usize, usize)>) {
}

#[cfg(debug_assertions)]
fn print_dots(dots: Vec<(usize, usize)>) {
    let mut dots_fmt = String::new();
    for y in 0..=dots.iter().map(|x| x.1).max().unwrap() {
        for x in 0..=dots.iter().map(|x| x.0).max().unwrap() {
            if dots.iter().contains(&(x, y)) {
                dots_fmt.push('#');
            } else {
                dots_fmt.push(' ');
            }
        }
        dots_fmt.push('\n');
    }
    println!("{}", dots_fmt);
}
