pub use self::Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);
pub const CARDINALS: [Direction; 4] = [North, West, South, East];
pub const DIAGONALS: [Direction; 4] = [NorthWest, NorthEast, SouthWest, SouthEast];

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

impl Point {
    pub fn adj(&self) -> impl Iterator<Item = Self> + '_ {
        get_all_adjacents().map(|x| x.apply(self))
    }
    pub fn cardinal_adj(&self) -> impl Iterator<Item = Self> + '_ {
        DIAGONALS.iter().map(|x| x.apply(self))
    }
    pub fn diagonal_adj(&self) -> impl Iterator<Item = Self> + '_ {
        DIAGONALS.iter().map(|x| x.apply(self))
    }
}

impl Direction {
    pub fn apply(&self, point: &Point) -> Point {
        let Point(dx, dy) = self.to_tuple();
        Point(point.0 + dx, point.1 + dy)
    }
    pub const fn to_tuple(&self) -> Point {
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
    pub fn as_index(&self, width: isize) -> isize {
        let Point(x, y) = self.to_tuple();
        as_index(x, y, width)
    }
    pub fn to_indices(&self, point: Point, width: isize, height: isize) -> Option<isize> {
        let Point(x, y) = self.to_tuple();
        let dx = x + point.0;
        let dy = y + point.0;

        if (0..width).contains(&dx) && (0..height).contains(&dy) {
            Some(as_index(dx, dy, width))
        } else {
            None
        }
    }
}

pub fn get_all_adjacents() -> impl Iterator<Item = Direction> {
    CARDINALS.into_iter().chain(DIAGONALS.into_iter())
}

fn as_index(x: isize, y: isize, width: isize) -> isize {
    x + y * width
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
