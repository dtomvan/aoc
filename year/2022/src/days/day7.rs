use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let mut map = HashMap::new();
    let mut pwd = PathBuf::from("/");

    for line in include_str!("../../inputs/day-7.txt").lines() {
        if line.starts_with("$ cd ") {
            match &line[5..] {
                ".." => {
                    pwd.pop();
                }
                "/" => {}
                x => pwd.push(x.to_string()),
            }
        } else if line.starts_with(|x: char| x.is_numeric()) {
            let l = line.split_once(' ').unwrap();
            if let Some(size) = l.0.parse().ok() {
                map.entry(format!("{}/{}", pwd.display(), l.1))
                    .or_insert_with(|| Node::File(size));
            }
        } else if line.starts_with("dir") {
            map.entry(format!("{}/{}", pwd.display().to_string(), &line[4..]))
                .or_insert(Node::Dir);
        }
    }

    let struc = std::iter::once((&"/".to_string(), &Node::Dir))
        .chain(map.iter().filter(|x| matches!(x.1, Node::Dir)))
        .map(|x| {
            map.iter()
                .filter(|y| y.0.starts_with(x.0.as_str()))
                .map(|x| if let Node::File(s) = x.1 { s } else { &0 })
                .sum::<usize>()
        })
        .collect_vec();

    // Part 2
    let total_size = struc.iter().max().unwrap();
    let free_space = 70000000 - total_size;
    let needed = 30000000 - free_space;
    done(
        // Part 1
        struc.iter().filter(|x| x <= &&100000).copied().ssum(),
        *struc
            .iter()
            .filter(|&&x| x >= needed)
            .sorted_unstable()
            .next()
            .unwrap(),
    )
}

enum Node {
    File(usize),
    Dir,
}
