/// From aoclib, don't want to add 180 crates just to use this code
/// Source: https://github.com/coriolinus/aoclib/blob/0db371840be4e/src/input.rs
use std::{
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader, Cursor},
    ops::Deref,
    path::Path,
    str::FromStr,
};

const TEST_DATA_FILENAME: &str = "TEST DATA";

pub fn parse<'a, T>(path: &'a Path) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_reader(
        reader,
        path.file_name()
            .expect("File::open() didn't early return before now; qed")
            .to_string_lossy(),
    )
}

pub fn parse_str<'a, T>(data: &'a str) -> std::io::Result<impl '_ + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    parse_reader(Cursor::new(data), TEST_DATA_FILENAME)
}

pub fn parse_reader<'a, T, Reader, Filename>(
    mut reader: Reader,
    file_name: Filename,
) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    Reader: 'a + BufRead,
    Filename: 'a + std::fmt::Display,
{
    let mut buf = String::new();
    let mut line: usize = 0;
    Ok(std::iter::from_fn(move || {
        buf.clear();
        reader.read_line(&mut buf).ok().and_then(|_| {
            line += 1;
            (!buf.is_empty())
                .then(|| match T::from_str(buf.trim()) {
                    Ok(t) => Some(t),
                    Err(e) => {
                        eprintln!("{}:{}: {} for {:?}", file_name, line, e, buf.trim());
                        None
                    }
                })
                .flatten()
        })
    })
    .fuse())
}

pub fn parse_newline_sep<'a, T>(path: &'a Path) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_newline_sep_reader(
        reader,
        path.file_name()
            .expect("File::open() didn't early return before now; qed")
            .to_string_lossy(),
    )
}

pub fn parse_newline_sep_str<'a, T>(data: &'a str) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: 'a + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    parse_newline_sep_reader(Cursor::new(data), TEST_DATA_FILENAME)
}

fn is_new_field(buf: &str) -> bool {
    let patterns = ["\n\n", "\n\r\n"];
    patterns.iter().any(|pat| {
        buf.as_bytes()
            .iter()
            .rev()
            .zip(pat.as_bytes().iter())
            .all(|(b, p)| b == p)
    })
}

fn get_next_item<T>(
    buf: &mut String,
    line: &mut usize,
    reader: &mut impl BufRead,
    file_name: &impl Display,
) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    buf.clear();
    while buf.is_empty() || !is_new_field(buf) {
        *line += 1;
        if reader.read_line(buf).ok()? == 0 {
            break;
        }
    }
    (!buf.is_empty())
        .then(|| match T::from_str(buf) {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("{}:{}: {} for {:?}", file_name, *line - 1, e, buf);
                None
            }
        })
        .flatten()
}

pub fn parse_newline_sep_reader<'a, T, Reader, Filename>(
    mut reader: Reader,
    file_name: Filename,
) -> std::io::Result<impl 'a + Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    Reader: 'a + BufRead,
    Filename: 'a + std::fmt::Display,
{
    let mut buf = String::new();
    let mut line: usize = 0;

    Ok(
        std::iter::from_fn(move || get_next_item(&mut buf, &mut line, &mut reader, &file_name))
            .fuse(),
    )
}

#[derive(Debug, thiserror::Error)]
pub enum TwoPhaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no first line")]
    NoFirstLine,
}

pub fn parse_two_phase_reader<'a, A, B, Reader, Filename>(
    mut reader: Reader,
    file_name: Filename,
) -> Result<(A, impl 'a + Iterator<Item = B>), TwoPhaseError>
where
    A: 'a + FromStr,
    <A as FromStr>::Err: Display,
    B: 'a + FromStr,
    <B as FromStr>::Err: Display,
    Reader: 'a + BufRead,
    Filename: 'a + Display,
{
    let mut buf = String::new();
    let mut line: usize = 0;

    let a = get_next_item(&mut buf, &mut line, &mut reader, &file_name)
        .ok_or(TwoPhaseError::NoFirstLine)?;

    Ok((
        a,
        std::iter::from_fn(move || get_next_item(&mut buf, &mut line, &mut reader, &file_name))
            .fuse(),
    ))
}

pub fn parse_two_phase<'a, A, B>(
    path: &'a Path,
) -> Result<(A, impl 'a + Iterator<Item = B>), TwoPhaseError>
where
    A: 'a + FromStr,
    <A as FromStr>::Err: Display,
    B: 'a + FromStr,
    <B as FromStr>::Err: Display,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_two_phase_reader(
        reader,
        path.file_name()
            .expect("File::open() didn't early return before now; qed")
            .to_string_lossy(),
    )
}

pub fn parse_two_phase_str<'a, A, B>(
    data: &'a str,
) -> Result<(A, impl 'a + Iterator<Item = B>), TwoPhaseError>
where
    A: 'a + FromStr,
    <A as FromStr>::Err: Display,
    B: 'a + FromStr,
    <B as FromStr>::Err: Display,
{
    parse_two_phase_reader(Cursor::new(data), TEST_DATA_FILENAME)
}

pub struct CommaSep<T>(Vec<T>);

impl<T> FromStr for CommaSep<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(CommaSep)
    }
}

impl<T> IntoIterator for CommaSep<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<CommaSep<T>> for Vec<T> {
    fn from(cs: CommaSep<T>) -> Self {
        cs.0
    }
}

impl<T> Deref for CommaSep<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TrimmedCommaSep<T>(Vec<T>);

impl<T> FromStr for TrimmedCommaSep<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(TrimmedCommaSep)
    }
}

impl<T> IntoIterator for TrimmedCommaSep<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<TrimmedCommaSep<T>> for Vec<T> {
    fn from(tcs: TrimmedCommaSep<T>) -> Self {
        tcs.0
    }
}

impl<T> Deref for TrimmedCommaSep<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn double_split(
    s: &'static str,
    first: &'static str,
    second: &'static str,
) -> impl Iterator<Item = impl Iterator<Item = &'static str>> {
    s.split(first).map(move |x| x.split(second))
}

pub fn double_split_ws(
    s: &'static str,
    first: &'static str,
) -> impl Iterator<Item = impl Iterator<Item = &'static str>> {
    s.split(first).map(move |x| x.split_whitespace())
}

pub fn double_split_grouped(
    s: &'static str,
) -> impl Iterator<Item = impl Iterator<Item = &'static str>> {
    s.split("\n\n").map(move |x| x.split_whitespace())
}

#[macro_export]
macro_rules! parse {
    ($type:ty) => {
        |x| x.parse::<$type>().ok()
    };
    () => {
        |x| x.parse().ok()
    };
}

#[macro_export]
macro_rules! lines {
    ($file:literal) => {
        include_str!($file).lines()
    };
}

#[macro_export]
macro_rules! chars {
    ($file:literal) => {
        include_str!($file).trim().chars()
    };
}

