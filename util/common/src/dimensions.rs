use std::ops::RangeInclusive;

use crate::point::Point;

macro_rules! adj_impl {
    ($($name:ident),+) => {
        $(pub fn $name(&self, p: Point) -> Option<impl Iterator<Item = Point> + '_> {
            if self.bounds(&p) {
                Some(p.$name().filter(|x| self.bounds(&x)))
            } else {
                None
            }
        })+
    };
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    /// 1st, 2nd, 3rd, 4th
    /// top-right, top-left, bottom-left, bottom-right
    pub quadrants: [bool; 4],
    pub(crate) rep: Option<isize>,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, quadrants: [bool; 4]) -> Self {
        assert!(width < isize::MAX as usize);
        assert!(height < isize::MAX as usize);

        Self {
            width,
            height,
            quadrants,
            rep: None,
        }
    }

    pub fn fnew(width: usize, height: usize, quadrants: impl Fn() -> [bool; 4]) -> Self {
        Self::new(width, height, quadrants())
    }

    pub fn bounds(&self, Point(x, y): &Point) -> bool {
        self.quadrants[match (x.is_negative(), y.is_negative()) {
            (true, true) => 2,
            (true, false) => 1,
            (false, true) => 3,
            (false, false) => 0,
        }] && (x.abs() < self.pseudo_width())
            && (y.abs() < self.pseudo_height())
    }

    pub fn n_quadrants(&self) -> usize {
        self.quadrants.iter().filter(|x| **x).count()
    }

    pub fn index(&self, p: Point) -> Option<usize> {
        let Point(x, y) = p + self.map();
        if self.bounds(&p) {
            (x + y * self.total_width()).try_into().ok()
        } else {
            None
        }
    }

    pub fn point(&self, i: usize) -> Option<Point> {
        i.try_into().ok().and_then(|i: isize| {
            let total_width = self.total_width();
            let p = Point(i % total_width, i / total_width) - self.map();

            if self.bounds(&p) {
                Some(p)
            } else {
                None
            }
        })
    }

    fn map(&self) -> Point {
        Point(
            if self.quadrants[1] || self.quadrants[2] {
                self.pseudo_width()
            } else {
                0
            },
            if self.quadrants[2] || self.quadrants[3] {
                self.pseudo_height()
            } else {
                0
            },
        )
    }

    adj_impl!(adj, cardinal_adj, diagonal_adj);

    pub(crate) fn x_range(&self) -> RangeInclusive<isize> {
        let pseudo_width = self.pseudo_width();

        if self.quadrants[1] || self.quadrants[2] {
            -(pseudo_width)..=pseudo_width
        } else {
            0..=pseudo_width
        }
    }

    pub(crate) fn y_range(&self) -> RangeInclusive<isize> {
        let pseudo_height = self.pseudo_height();

        if self.quadrants[2] || self.quadrants[3] {
            -(pseudo_height)..=pseudo_height
        } else {
            0..=pseudo_height
        }
    }

    pub fn rep_amount(&self, p: Point) -> Point {
        p / Point(self.width as isize, self.height as isize)
    }

    #[inline]
    #[must_use]
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    #[must_use]
    pub fn w_h(&self) -> Point {
        Point(self.pseudo_width(), self.pseudo_height())
    }

    #[inline(always)]
    #[must_use]
    fn total_width(&self) -> isize {
        let q = self.n_quadrants() as isize;
        let w = self.pseudo_width();
        if q != 1 {
            w * q
        } else {
            w
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn pseudo_width(&self) -> isize {
        if let Some(rep) = self.rep {
            rep * self.width as isize
        } else {
            self.width as _
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn pseudo_height(&self) -> isize {
        if let Some(rep) = self.rep {
            rep * self.height as isize
        } else {
            self.height as _
        }
    }
}

pub const fn q_all() -> [bool; 4] {
    [true; 4]
}

pub const fn q_none() -> [bool; 4] {
    [false; 4]
}

pub const fn q_pos() -> [bool; 4] {
    [true, false, false, false]
}

pub const fn q_neg() -> [bool; 4] {
    [false, false, true, false]
}
