// DOESNT WORK
use aoc_common::prelude::*;
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-15.test")
        .lines()
        .map(|x| x.chars().flat_map(|x| x.to_digit(10)).collect_vec())
        .collect_vec();

    let d = Dimensions::fnew(input[0].len(), input.len(), q_pos);

    let flat_i = input.iter().flatten().collect_vec();

    let adj_list: Graph = (0..flat_i.len())
        .into_iter()
        .map(|i| {
            let p = d.point(i).unwrap();

            CARDINALS
                .into_iter()
                .filter_map(|dir| {
                    d.index(p + dir).and_then(|j| {
                        if j != i {
                            Some(Edge {
                                node: p.0 as usize,
                                cost: *flat_i[p.0 as usize] as isize,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect_vec()
        })
        .collect_vec();

    let last = flat_i.len() - 1;
    let part_1 = dijkstra(adj_list, |state| state.position == last);

    done((part_1 - *flat_i[0] as isize) as usize, ())
}
