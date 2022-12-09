use std::{
    ops::{Index, Mul},
    str::FromStr,
};

use itertools::{iproduct, Itertools};

use crate::{
    collections::{OptionRes, Unavailable},
    dimensions::{q_pos, Dimensions},
    point::Point,
};

macro_rules! adj_impl {
    ($($name:ident),+) => {
        $(fn $name(&self, p: Point) -> Option<impl Iterator<Item = Point> + '_> {
            self.dimensions().$name(p)
        })+
    };
}

pub trait Plane {
    fn dimensions(&self) -> &Dimensions;

    fn points(&self) -> impl Iterator<Item = Point> {
        iproduct!(self.dimensions().x_range(), self.dimensions().y_range())
            .map(|(x, y)| Point(x, y))
    }

    fn column(&self, x: isize) -> impl Iterator<Item = Point> {
        self.dimensions().y_range().map(move |y| Point(x, y))
    }

    fn row(&self, y: isize) -> impl Iterator<Item = Point> {
        self.dimensions().x_range().map(move |x| Point(x, y))
    }

    fn column_values<T: Clone, U: FromIterator<T>>(&self, x: isize) -> U
    where
        Self: Index<Point, Output = T>,
    {
        self.dimensions()
            .y_range()
            .map(|y| self[Point(x, y)].clone())
            .collect()
    }

    fn row_values<T: Clone, U: FromIterator<T>>(&self, y: isize) -> U
    where
        Self: Index<Point, Output = T>,
    {
        self.dimensions()
            .x_range()
            .map(|x| self[Point(x, y)].clone())
            .collect()
    }

    adj_impl!(adj, cardinal_adj, diagonal_adj);
}

// pub struct PlaneDisplay<
//     'a,
//     T: Display + 'a,
//     U: Plane + SliceIndex<>,
// >(&'a U);
//
// impl<
//         'a,
//         T: Display + 'a,
//         U: Plane + SliceIndex<T>,
//     > Display for PlaneDisplay<'a, T, U>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let wi = self.0.dimensions().pseudo_width();
//         let mut w = 0;
//         for val in self.0.into_iter() {
//             write!(f, "{val}")?;
//             w += 1;
//             if w == wi {
//                 w = 0;
//                 writeln!(f)?;
//             }
//         }
//         Ok(())
//     }
// }

pub type IntGrid = Grid<isize>;
pub type UIntGrid = Grid<usize>;
pub type BoolGrid = Grid<bool>;
pub type CharGrid = Grid<char>;

// (0, 0) to (width, height)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    pub dimensions: Dimensions,
    pub data: Vec<T>,
}

impl CharGrid {
    pub fn chars(s: &'static str) -> Result<Self, Unavailable> {
        Self::try_from(
            s.trim()
                .lines()
                .map(|x| x.chars().collect_vec())
                .collect_vec(),
        )
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            dimensions: Dimensions::fnew(width, height, q_pos),
            data,
        }
    }

    pub fn multi(self, rep: isize) -> PseudoGrid<T> {
        self * rep
    }
}

impl<T> Plane for Grid<T> {
    #[inline]
    fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[self.dimensions.index(index).expect("Point is invalid")]
    }
}

impl<T> Mul<isize> for Grid<T> {
    type Output = PseudoGrid<T>;

    fn mul(self, rhs: isize) -> Self::Output {
        PseudoGrid::new(self, rhs)
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            dimensions: Default::default(),
            data: Vec::new(),
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = Unavailable;

    fn try_from(vec: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = vec.len();
        let width = vec.first().map(|x| x.len());

        width.why("empty input data for grid").and_then(|width| {
            if vec.iter().map(|x| x.len()).all_equal() {
                Some(Self::new(
                    width,
                    height,
                    vec.into_iter().flatten().collect_vec(),
                ))
            } else {
                None
            }
            .why("inequal widths")
        })
    }
}

impl<T: FromStr> TryFrom<&'static str> for Grid<T> {
    type Error = Unavailable;

    fn try_from(v: &'static str) -> Result<Self, Self::Error> {
        Self::try_from(
            v.trim()
                .lines()
                .map(|x| {
                    x.chars()
                        .filter_map(|x| x.encode_utf8(&mut [0; 8]).parse::<T>().ok())
                        .collect_vec()
                })
                .collect_vec(),
        )
    }
}

// (0, 0) to (width * rep, height * rep)
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PseudoGrid<T>(pub Grid<T>);

impl<T> PseudoGrid<T> {
    pub fn new(mut grid: Grid<T>, rep: isize) -> Self {
        grid.dimensions.rep = Some(rep);
        Self(grid)
    }
}

impl<T> Plane for PseudoGrid<T> {
    #[inline]
    fn dimensions(&self) -> &Dimensions {
        &self.0.dimensions
    }
}

impl<T> Index<Point> for PseudoGrid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[self.0.dimensions.index(index).expect("Point is invalid")
            % self
                .0
                .dimensions
                .rep
                .expect("Should be set in PseudoGrid::new") as usize]
    }
}

impl<T> Index<usize> for PseudoGrid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.data.index(
            self.0
                .dimensions
                .index(self.0.dimensions.point(index).expect("Point is invalid"))
                .unwrap()
                % self
                    .0
                    .dimensions
                    .rep
                    .expect("Should be set in PseudoGrid::new") as usize,
        )
    }
}
