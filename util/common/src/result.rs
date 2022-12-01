use std::fmt::Display;

use num_bigint::{BigInt, BigUint};

pub fn done<A: Into<AocD>, B: Into<AocD>>(a: A, b: B) -> AocResult {
    Ok((a.into(), b.into()))
}

pub fn done_second<A: Into<AocD> + Clone, B: Into<AocD>>(a: A, b: &mut dyn FnMut(A) -> B) -> AocResult {
    Ok((a.clone().into(), b(a).into()))
}

pub type AocResult = anyhow::Result<(AocD, AocD)>;

#[derive(Default, Debug, Clone)]
pub enum AocD {
    #[default]
    Nothing,
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
            Nothing => write!(f, "TODO"),
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

impl From<()> for AocD {
    fn from(_: ()) -> Self {
        Self::Nothing
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
