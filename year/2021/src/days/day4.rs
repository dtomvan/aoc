use std::str::FromStr;

use aoc_common::prelude::*;
use itertools::Itertools;

pub fn main() -> AocResult {
    let mut input = include_str!("../../inputs/day-4.txt").split("\n\n");

    let draws: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .filter_map(parse!())
        .collect();
    let mut boards = input.filter_map(parse!(Board)).collect_vec();

    let mut part_1 = None;
    let mut part_2 = None;
    for draw in draws {
        boards.drain_filter(|board| {
            board
                .iter_mut()
                .filter(|x| x.has(draw))
                .for_each(Cell::fill);
            let won = board.won();
            if won {
                let score = draw
                    * board
                        .iter()
                        .filter_map(|x| {
                            if let Cell::Empty(n) = x {
                                Some(n)
                            } else {
                                None
                            }
                        })
                        .sum::<usize>();
                part_1.get_or_insert(score);
                let _ = part_2.insert(score);
            }
            won
        });
    }

    done(part_1.unwrap(), part_2.unwrap())
}

#[derive(Clone, Debug)]
struct Board(Vec<Vec<Cell>>);

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| Cell::Empty(x.parse().unwrap()))
                        .collect()
                })
                .collect(),
        ))
    }
}

impl Board {
    fn rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.0.iter()
    }
    fn cols(&self) -> impl Iterator<Item = Vec<&Cell>> {
        Cols::new(&self.0)
    }
    fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.0.iter().flat_map(|x| x.iter())
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.0.iter_mut().flat_map(|x| x.iter_mut())
    }
    fn won(&self) -> bool {
        self.rows()
            .any(|x| is_filled(&x.iter().collect::<Vec<_>>()[..]))
            || self.cols().any(|x| is_filled(&x[..]))
    }
}

struct Cols<'a> {
    vec: &'a [Vec<Cell>],
    index: usize,
}

impl<'a> Cols<'a> {
    const fn new(vec: &'a [Vec<Cell>]) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'a> Iterator for Cols<'a> {
    type Item = Vec<&'a Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Vec<_> = self.vec.iter().filter_map(|x| x.get(self.index)).collect();
        self.index += 1;
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Filled,
    Empty(usize),
}

impl Cell {
    fn fill(&mut self) {
        *self = Self::Filled;
    }
    const fn filled(&self) -> bool {
        matches!(self, Self::Filled)
    }
    fn has(&self, val: usize) -> bool {
        if let Self::Empty(x) = self {
            x == &val
        } else {
            false
        }
    }
}

fn is_filled(vec: &[&Cell]) -> bool {
    vec.iter().all(|x| x.filled())
}
