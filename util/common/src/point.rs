pub use self::Direction::*;

use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub},
    str::FromStr,
};
use crate::{collections::Unavailable, dimensions::Dimensions};

use itertools::Itertools;
use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    From, Mul, MulAssign, Neg, Not, Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign, Sum,
};
use paste::paste;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Add,
    AddAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    From,
    Mul,
    MulAssign,
    Neg,
    Not,
    Product,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Sub,
    SubAssign,
    Sum,
)]
#[mul(forward)]
#[div(forward)]
pub struct Point(pub isize, pub isize);

pub const CARDINALS: [Direction; 4] = [North, West, South, East];
pub const DIAGONALS: [Direction; 4] = [NorthWest, NorthEast, SouthWest, SouthEast];

macro_rules! is_adj_impl {
    ($($f:ident),+) => {
        $(paste!{pub fn [<is_ $f>](self, other: &Self) -> bool {
            self.$f().contains(other)
        }})+
    };
}

impl Point {
    #[inline]
    pub fn t(self) -> (isize, isize) {
        (self.0, self.1)
    }
    pub fn adj(self) -> impl Iterator<Item = Self> + 'static {
        self.cardinal_adj().chain(self.diagonal_adj())
    }
    pub fn cardinal_adj(self) -> impl Iterator<Item = Self> + 'static {
        CARDINALS.into_iter().map(move |x| self + x)
    }
    pub fn diagonal_adj(self) -> impl Iterator<Item = Self> + 'static {
        DIAGONALS.into_iter().map(move |x| self + x)
    }
    is_adj_impl!(adj, cardinal_adj, diagonal_adj);
    pub fn clamp(self, bx: isize, by: isize) -> Self {
        Self(min(max(self.0, -bx), bx), min(max(self.1, -by), by))
    }
    pub fn clamp_b(self, min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> Self {
        Self(
            min(max(self.0, min_x), max_x),
            min(max(self.1, min_y), max_y),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    // Cardinals
    North,
    West,
    South,
    East,
    // Diagonals
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, d: Direction) -> Self::Output {
        let Point(x, y) = self;
        let Point(dx, dy) = d.to_point();
        Point(x + dx, y + dy)
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, d: Direction) {
        let Point(dx, dy) = d.to_point();
        self.0 += dx;
        self.1 += dy;
    }
}

impl Sub<isize> for Point {
    type Output = Point;

    fn sub(self, d: isize) -> Self::Output {
        let Point(x, y) = self;
        Point(x - d, y - d)
    }
}

impl Div<isize> for Point {
    type Output = Point;

    fn div(self, d: isize) -> Self::Output {
        let Point(x, y) = self;
        Point(x / d, y / d)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, d: isize) -> Self::Output {
        let Point(x, y) = self;
        Point(x * d, y * d)
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, d: Point) {
        let Point(dx, dy) = d;
        self.0 *= dx;
        self.1 *= dy;
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, d: Point) {
        let Point(dx, dy) = d;
        self.0 /= dx;
        self.1 /= dy;
    }
}

impl Direction {
    pub const fn to_point(&self) -> Point {
        match self {
            North => Point(0, -1),
            West => Point(-1, 0),
            South => Point(0, 1),
            East => Point(1, 0),
            NorthWest => Point(-1, -1),
            NorthEast => Point(1, -1),
            SouthWest => Point(-1, 1),
            SouthEast => Point(1, 1),
        }
    }
    pub const fn flipped_x(self) -> Self {
        match self {
            West => East,
            East => West,
            x => x,
        }
    }
    pub const fn flipped_y(self) -> Self {
        match self {
            North => South,
            South => North,
            x => x,
        }
    }
    pub fn from_point_clamped(p: Point) -> Self {
        Self::try_from(p.clamp(1, 1)).unwrap()
    }
    pub fn index(&self, p: Point, dimensions: Dimensions) -> Option<usize> {
        dimensions.index(p + self.to_point())
    }
    pub fn from_index(index: usize, dimensions: Dimensions) -> Option<Self> {
        dimensions.point(index).and_then(|x| x.try_into().ok())
    }
}

impl FromStr for Direction {
    type Err = Unavailable;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::North),
            "D" => Ok(Direction::South),
            "L" => Ok(Direction::West),
            "R" => Ok(Direction::East),
            _ => Err(Unavailable("Expected one of U, D, L, R")),
        }
    }
}

impl Add<Direction> for Direction {
    type Output = Option<Direction>;

    fn add(self, d: Direction) -> Self::Output {
        let Point(dx, dy) = self.to_point();
        let Point(dx2, dy2) = d.to_point();
        Direction::try_from(Point(dx + dx2, dy + dy2)).ok()
    }
}

impl std::convert::TryFrom<Point> for Direction {
    type Error = &'static str;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        match value {
            Point(0, -1) => Ok(North),
            Point(0, 1) => Ok(South),
            Point(1, 0) => Ok(East),
            Point(-1, 0) => Ok(West),
            Point(-1, -1) => Ok(NorthWest),
            Point(1, -1) => Ok(NorthEast),
            Point(-1, 1) => Ok(SouthWest),
            Point(1, 1) => Ok(SouthEast),
            _ => Err("This isn't a Direction"),
        }
    }
}
