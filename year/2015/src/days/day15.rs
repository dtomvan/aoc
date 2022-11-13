use std::{
    cmp::max,
    collections::{hash_map::RandomState, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc_common::result::{AocResult, done};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-15.txt");
    let puzzle_input = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .skip(2)
                .step_by(2)
                .filter_map(|x| x.replace(',', "").parse().ok())
                .collect_vec()
                .into()
        })
        .collect_vec();
    let mut best_cookie = ScoredCookie::default();
    let mut queue = VecDeque::new();
    let starter_cookie = ScoredCookie::new(Cookie::equal_dist(puzzle_input));
    queue.push_back(starter_cookie.clone());
    let mut seen: HashSet<_, RandomState> = HashSet::from([starter_cookie.hash()]);
    while let Some(cookie) = queue.pop_back() {
        for not_seen in cookie
            .cookie
            .adjacent()
            .into_iter()
            .map(ScoredCookie::new)
            .filter(|x| seen.insert(x.hash()))
        {
            queue.push_back(not_seen);
        }
        best_cookie = max(best_cookie, cookie);
    }
    // Part 2
    done(best_cookie.score, ())
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ScoredCookie {
    score: usize,
    cookie: Cookie,
}

impl PartialOrd for ScoredCookie {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredCookie {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ScoredCookie {
    fn new(cookie: Cookie) -> Self {
        Self {
            score: cookie.score(),
            cookie,
        }
    }

    fn hash(&self) -> isize {
        self.cookie.hash()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Cookie(HashMap<Ingredient, isize>);

impl Cookie {
    fn hash(&self) -> isize {
        self.0
            .values()
            .enumerate()
            .fold(0, |acc, (i, el)| acc | (el & 0x7F << (i * 7)))
    }
    fn adjacent(&self) -> Vec<Cookie> {
        self.0
            .iter()
            .tuple_combinations()
            .flat_map(|((in_a, p_a), (in_b, p_b))| {
                let mut c_1 = self.clone().0;
                c_1.insert(*in_a, p_a + 1);
                c_1.insert(*in_b, p_b - 1);
                let mut c_2 = self.clone().0;
                c_2.insert(*in_a, p_a - 1);
                c_2.insert(*in_b, p_b + 1);
                vec![Cookie(c_1), Cookie(c_2)]
            })
            .filter(|x| x.0.iter().all(|x| x.1 > &0))
            .collect_vec()
    }
    fn equal_dist(i: Vec<Ingredient>) -> Self {
        let eq_dist = 100 / i.len() as isize;
        Self(i.into_iter().map(|x| (x, eq_dist)).collect())
    }
    fn single_ingredient(&self) -> Ingredient {
        self.0
            .iter()
            .fold(Ingredient::default(), |acc, (i, p)| Ingredient {
                capacity: acc.capacity + i.capacity * p,
                durability: acc.durability + i.durability * p,
                flavor: acc.flavor + i.flavor * p,
                texture: acc.texture + i.texture * p,
                calories: acc.calories + i.calories * p,
            })
    }
    fn score(&self) -> usize {
        let s = self.single_ingredient();
        s.capacity.bind_zero()
            * s.durability.bind_zero()
            * s.flavor.bind_zero()
            * s.texture.bind_zero()
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl From<Vec<isize>> for Ingredient {
    fn from(v: Vec<isize>) -> Self {
        assert_eq!(v.len(), 5, "Invalid puzzle input");
        Self {
            capacity: v[0],
            durability: v[1],
            flavor: v[2],
            texture: v[3],
            calories: v[4],
        }
    }
}

trait BindZero: Copy {
    fn bind_zero(self) -> usize;
}

impl BindZero for isize {
    fn bind_zero(self) -> usize {
        self.max(0).unsigned_abs()
    }
}
