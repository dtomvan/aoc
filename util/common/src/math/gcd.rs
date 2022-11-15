use std::{
    cmp::min,
    mem::swap,
    ops::{Add, Div, Rem, Sub},
};

use num_traits::Unsigned;

/// Way to generic gcd and lcm function
pub fn gcd<T: Gcd>(mut a: T, mut b: T) -> T {
    if a < b {
        swap(&mut a, &mut b);
    }

    let def = T::default();
    let two = T::from(2);
    match (a, b) {
        (a, b) if b == def => a,
        (a, b) if a == def => b,
        _ => match (a % two, b % two) {
            (def, o) if def == o => two * gcd(a / two, b / two),
            (a, _) if a == def => gcd(a / two, b),
            (_, b) if b == def => gcd(a, b / two),
            _ => gcd(a - b, min(a, b)),
        },
    }
}

pub fn lcm<T: Gcd>(a: T, b: T) -> T
{
    (a * b) / gcd(a, b)
}

pub trait Gcd: Add + Copy + Default + Div + From<u8> + Ord + PartialOrd + Rem + Sub + Unsigned {}
impl Gcd for u8 {}
impl Gcd for u16 {}
impl Gcd for u32 {}
impl Gcd for u64 {}
impl Gcd for u128 {}
impl Gcd for usize {}
