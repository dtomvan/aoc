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
    width: usize,
    height: usize,
    /// 1st, 2nd, 3rd, 4th
    /// top-right, top-left, bottom-left, bottom-right
    quadrants: [bool; 4],
}

impl Dimensions {
    pub fn new(width: usize, height: usize, quadrants: [bool; 4]) -> Self {
        assert!(width < isize::MAX as usize);
        assert!(height < isize::MAX as usize);

        Self {
            width,
            height,
            quadrants,
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
        }] && (x.abs() < self.width as isize)
            && (y.abs() < self.height as isize)
    }

    pub fn n_quadrants(&self) -> usize {
        self.quadrants.iter().filter(|x| **x).count()
    }

    pub fn index(&self, p: Point) -> Option<usize> {
        let Point(x, y) = p + self.map();
        if self.bounds(&p) {
            (x + y * self.width as isize * self.n_quadrants() as isize / 2)
                .try_into()
                .ok()
        } else {
            None
        }
    }

    pub fn point(&self, i: usize) -> Option<Point> {
        i.try_into().ok().and_then(|i: isize| {
            let total_width = (self.width * self.n_quadrants() / 2) as isize;
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
                self.width as isize
            } else {
                0
            },
            if self.quadrants[2] || self.quadrants[3] {
                self.height as isize
            } else {
                0
            },
        )
    }

    adj_impl!(adj, cardinal_adj, diagonal_adj);
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
