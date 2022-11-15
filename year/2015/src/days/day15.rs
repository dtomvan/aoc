use std::{
    collections::{hash_map::RandomState, BTreeMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc_common::result::{done, AocResult};
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
    let part_1 = part(false, puzzle_input.clone());
    let part_2 = part(true, puzzle_input);
    // Part 2
    done(part_1.score, part_2.score)
}

fn part(two: bool, input: Vec<Ingredient>) -> ScoredCookie {
    let mut best_cookie = ScoredCookie::default();
    let mut queue = VecDeque::new();
    let starter_cookie = ScoredCookie::new(Cookie::equal_dist(input));
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
        if cookie > best_cookie && (!two || cookie.calories == 500) {
            best_cookie = cookie;
        }
    }
    best_cookie
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScoredCookie {
    score: usize,
    calories: usize,
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
    pub fn new(cookie: Cookie) -> Self {
        let (score, calories) = cookie.score();
        Self {
            score,
            calories,
            cookie,
        }
    }

    pub fn hash(&self) -> isize {
        self.cookie
            .0
            .values()
            .enumerate()
            .fold(0, |acc, (i, el)| acc | ((el & 0x7F) << (i * 7)))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cookie(pub BTreeMap<Ingredient, isize>);

impl Cookie {
    pub fn adjacent(&self) -> Vec<Cookie> {
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
    pub fn equal_dist(i: Vec<Ingredient>) -> Self {
        let eq_dist = 100 / i.len() as isize;
        Self(i.into_iter().map(|x| (x, eq_dist)).collect())
    }
    pub fn single_ingredient(&self) -> Ingredient {
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
    pub fn score(&self) -> (usize, usize) {
        let s = self.single_ingredient();
        (
            s.capacity.bind_zero()
                * s.durability.bind_zero()
                * s.flavor.bind_zero()
                * s.texture.bind_zero(),
            s.calories.bind_zero(),
        )
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ingredient {
    pub capacity: isize,
    pub durability: isize,
    pub flavor: isize,
    pub texture: isize,
    pub calories: isize,
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
