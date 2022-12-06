pub use anyhow::Result;

use std::{fmt::Display, iter::Sum};
use num_bigint::{BigInt, BigUint};

pub fn done<A: Into<AocD>, B: Into<AocD>>(a: A, b: B) -> AocResult {
    Ok((a.into(), b.into()))
}

pub fn done_second<A: Into<AocD> + Clone, B: Into<AocD>>(
    a: A,
    b: &mut dyn FnMut(A) -> B,
) -> AocResult {
    Ok((a.clone().into(), b(a).into()))
}

pub type AocResult = Result<(AocD, AocD)>;

#[derive(Default, Debug, Clone)]
pub enum AocD {
    #[default]
    TODO,
    Nothing,
    U16(u16),
    Number(usize),
    ULong(BigUint),
    Integer(isize),
    Long(BigInt),
    String(String),
    List(Vec<AocD>),
}

impl Display for AocD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AocD::*;
        match self {
            TODO => write!(f, "TODO"),
            Nothing => write!(f, "No solution :("),
            U16(n) => write!(f, "{n}"),
            Integer(n) => write!(f, "{n}"),
            Long(n) => write!(f, "{n}"),
            Number(n) => write!(f, "{n}"),
            String(s) => write!(f, "{s}"),
            ULong(n) => write!(f, "{n}"),
            List(l) => {
                write!(f, "[")?;
                let mut second = false;
                for x in l {
                    if second {
                        write!(f, ", ")?;
                    } else {
                        second = true;
                    }
                    write!(f, "{x}")?;
                }
                write!(f, "]")
            }
        }
    }
}

impl From<u16> for AocD {
    fn from(v: u16) -> Self {
        Self::U16(v)
    }
}

impl From<()> for AocD {
    fn from(_: ()) -> Self {
        Self::TODO
    }
}

impl From<Vec<AocD>> for AocD {
    fn from(v: Vec<AocD>) -> Self {
        Self::List(v)
    }
}

impl From<String> for AocD {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<BigInt> for AocD {
    fn from(v: BigInt) -> Self {
        Self::Long(v)
    }
}

impl From<isize> for AocD {
    fn from(v: isize) -> Self {
        Self::Integer(v)
    }
}

impl From<BigUint> for AocD {
    fn from(v: BigUint) -> Self {
        Self::ULong(v)
    }
}

impl From<usize> for AocD {
    fn from(v: usize) -> Self {
        Self::Number(v)
    }
}

impl<T> From<Option<T>> for AocD
where
    AocD: From<T>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => Self::from(v),
            None => Self::Nothing,
        }
    }
}

/// Self-sum, extension fn to sum to the same type as the input
pub trait SSum<T: Sum>: Iterator<Item = T>
where
    Self: Sized,
{
    fn ssum(self) -> T {
        self.sum()
    }
}

macro_rules! ssum_impl {
    ($($ty:ty),+) => {
        $(impl<I: Iterator<Item = $ty>> SSum<$ty> for I {})+
    };
}

ssum_impl!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64);

pub trait BoolThen: Sized {
    fn then<T>(self, value: T) -> Option<T>;
    fn and_then<T>(self, value: impl FnMut() -> T) -> Option<T>;
}

impl BoolThen for bool {
    fn then<T>(self, value: T) -> Option<T> {
        if self {
            Some(value)
        } else {
            None
        }
    }

    fn and_then<T>(self, mut value: impl FnMut() -> T) -> Option<T> {
        if self {
            Some(value())
        } else {
            None
        }
    }
}
