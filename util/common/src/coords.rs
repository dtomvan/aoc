pub use self::Direction::*;

pub type Point = (isize, isize);
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

impl Direction {
    pub const fn to_tuple(&self) -> Point {
        match self {
            North => (0, -1),
            West => (-1, 0),
            South => (0, 1),
            East => (1, 0),
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthWest => (-1, 1),
            SouthEast => (1, 1),
        }
    }
    pub fn as_index(&self, width: isize) -> isize {
        let (x, y) = self.to_tuple();
        as_index(x, y, width)
    }
    pub fn to_indices(&self, point: Point, width: isize, height: isize) -> Option<isize> {
        let (x, y) = self.to_tuple();
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
            (0, -1) => Ok(North),
            (0, 1) => Ok(South),
            (1, 0) => Ok(East),
            (-1, 0) => Ok(West),

            (-1, -1) => Ok(NorthWest),
            (1, -1) => Ok(NorthEast),
            (-1, 1) => Ok(SouthWest),
            (1, 1) => Ok(SouthEast),
            _ => Err("This isn't a Direction"),
        }
    }
}
