pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v.iter().map(|x| x.len()).max().unwrap())
        .map(|i| {
            v.iter()
                .filter_map(|inner| inner.get(i).map(Clone::clone))
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn transpose_equal_len<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
