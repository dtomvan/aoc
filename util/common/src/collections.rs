pub mod tuples;

pub use tuples::*;

use itertools::Itertools;
use std::hash::Hash;
use thiserror::Error;

pub trait Transpose<T>: IntoIterator<Item = T> + Sized {
    fn transpose(self) -> Self;
    fn transpose_equal_len(self) -> Self;
}

impl<T> Transpose<Vec<T>> for Vec<Vec<T>>
where
    T: Clone,
{
    fn transpose(self) -> Self {
        assert!(!self.is_empty());
        (0..self.iter().map(|x| x.len()).max().unwrap())
            .map(|i| {
                self.iter()
                    .filter_map(|inner| inner.get(i).map(Clone::clone))
                    .collect()
            })
            .collect()
    }

    fn transpose_equal_len(self) -> Vec<Vec<T>> {
        assert!(!self.is_empty());
        let len = self[0].len();
        let mut iters: Vec<_> = self.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
            .collect()
    }
}

pub trait Rotate<T>: Sized {
    fn rotate(self, amount: usize) -> Self;
}

impl<T: Clone> Rotate<T> for Vec<T> {
    fn rotate(self, amount: usize) -> Self {
        let len = self.len();
        let modded = amount % len;
        let shift = if amount < 1 { len + modded } else { modded };
        if shift == 0 {
            self
        } else {
            let (l, r) = self.split_at(len - shift);
            [r, l].concat()
        }
    }
}

pub fn unique_window_start<T>(vec: &[T], len: usize) -> Option<(usize, &[T])>
where
    T: Hash + Eq,
{
    vec.windows(len)
        .find_position(|slice| slice.iter().all_unique())
}

pub fn unique_window_end<T>(vec: &[T], len: usize) -> Option<(usize, &[T])>
where
    T: Hash + Eq,
{
    unique_window_start(vec, len).map(|(n, t)| (n + len, t))
}

#[derive(Debug, Default, Clone, Error)]
#[error("Value is unavailable: `{0}`")]
pub struct Unavailable(pub &'static str);

pub trait OptionRes<T>: Sized {
    fn res(self) -> Result<T, Unavailable>;
    fn why(self, s: &'static str) -> Result<T, Unavailable>;
}

impl<T> OptionRes<T> for Option<T> {
    fn res(self) -> Result<T, Unavailable> {
        self.ok_or(Unavailable("None"))
    }
    fn why(self, s: &'static str) -> Result<T, Unavailable> {
        self.ok_or(Unavailable(s))
    }
}

#[macro_export]
macro_rules! vec_deque {
    [] => (
        ::std::collections::VecDeque::new()
    );
    [$elem:expr; $n:expr] => (
        let mut x = ::std::collections::VecDeque::new();
        x.push_front($n);
    );
    [$($x:expr),+ $(,)?] => (
        ::std::collections::VecDeque::from([$($x),+])
    );
}
