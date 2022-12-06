pub mod option_tuple;

pub use option_tuple::*;

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
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect()
            })
            .collect()
    }
}

pub fn unique_window_start<T>(vec: &Vec<T>, len: usize) -> Option<(usize, &[T])>
where
    T: Hash + Eq,
{
    vec.windows(len)
        .find_position(|slice| slice.into_iter().all_unique())
}

pub fn unique_window_end<T>(vec: &Vec<T>, len: usize) -> Option<(usize, &[T])>
where
    T: Hash + Eq,
{
    unique_window_start(vec, len).map(|(n, t)| (n + len, t))
}

#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct Unavailable(&'static str);

pub trait OptionRes<T>: Sized {
    fn res(self) -> Result<T, Unavailable>;
    fn why(self, s: &'static str) -> Result<T, Unavailable>;
}

impl<T> OptionRes<T> for Option<T> {
    fn res(self) -> Result<T, Unavailable> {
        self.ok_or_else(|| Unavailable("None"))
    }
    fn why(self, s: &'static str) -> Result<T, Unavailable> {
        self.ok_or_else(|| Unavailable(s))
    }
}
