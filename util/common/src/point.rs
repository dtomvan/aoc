use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    From, Mul, MulAssign, Neg, Not, Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign, Sum,
};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

use crate::dimensions::Dimensions;

pub use self::Direction::*;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
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
pub struct Point(pub isize, pub isize);

pub const CARDINALS: [Direction; 4] = [North, West, South, East];
pub const DIAGONALS: [Direction; 4] = [NorthWest, NorthEast, SouthWest, SouthEast];

impl Point {
    pub fn adj(self) -> impl Iterator<Item = Self> + 'static {
        get_all_adjacents().map(move |x| self + x)
    }
    pub fn cardinal_adj(self) -> impl Iterator<Item = Self> + 'static {
        DIAGONALS.into_iter().map(move |x| self + x)
    }
    pub fn diagonal_adj(self) -> impl Iterator<Item = Self> + 'static {
        DIAGONALS.into_iter().map(move |x| self + x)
    }
    pub fn as_index(&self, width: usize) -> Option<usize> {
        (self.0 + self.1 * (width as isize)).try_into().ok()
    }
}

#[derive(Debug)]
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

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, d: isize) -> Self::Output {
        let Point(x, y) = self;
        Point(x * d, y * d)
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, d: Point) -> Self::Output {
        let Point(x, y) = self;
        let Point(dx, dy) = d;
        Point(x / dx, y / dy)
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
    pub fn index(&self, p: Point, dimensions: Dimensions) -> Option<usize> {
        dimensions.index(p + self.to_point())
    }
    pub fn from_index(index: usize, dimensions: Dimensions) -> Option<Self> {
        dimensions.point(index).and_then(|x| x.try_into().ok())
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

pub fn get_all_adjacents() -> impl Iterator<Item = Direction> {
    CARDINALS.into_iter().chain(DIAGONALS.into_iter())
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