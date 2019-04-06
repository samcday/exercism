#![deny(clippy::all, clippy::pedantic)]

#[allow(clippy::needless_pass_by_value)]
pub fn find<V, T>(array: V, key: T) -> Option<usize>
where
    T: std::cmp::Ord,
    V: AsRef<[T]>,
{
    let array = array.as_ref();
    let mut range = 0..array.len();

    while range.start < range.end {
        let idx = range.start + (range.end - range.start) / 2;
        match array[idx].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(idx),
            std::cmp::Ordering::Greater => range.end = idx,
            std::cmp::Ordering::Less => range.start = idx + 1,
        }
    }

    None
}
