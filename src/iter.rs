#[cfg(feature = "alloc")]
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub trait IteratorExt: Iterator {
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[cfg(feature = "alloc")]
    fn join_(&mut self, sep: &str) -> String
    where
        Self::Item: fmt::Display,
    {
        use core::fmt::Write as _;

        match self.next() {
            None => String::new(),
            Some(first) => {
                let (lower, _) = self.size_hint();
                let mut buf = String::with_capacity(sep.len() * lower);
                write!(&mut buf, "{first}").unwrap();
                self.for_each(|item| {
                    buf.push_str(sep);
                    write!(&mut buf, "{item}").unwrap();
                });
                buf
            }
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

pub fn map_collect<C, T, I, F>(iterable: I, f: F) -> C
where
    I: IntoIterator,
    F: FnMut(I::Item) -> T,
    C: FromIterator<T>,
{
    iterable.into_iter().map(f).collect()
}

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
pub fn map_collect_vec<T, I, F>(iterable: I, f: F) -> Vec<T>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> T,
{
    map_collect(iterable, f)
}

pub fn filter_map_collect<C, T, I, F>(iterable: I, f: F) -> C
where
    I: IntoIterator,
    F: FnMut(I::Item) -> Option<T>,
    C: FromIterator<T>,
{
    iterable.into_iter().filter_map(f).collect()
}

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
pub fn filter_map_collect_vec<T, I, F>(iterable: I, f: F) -> Vec<T>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> Option<T>,
{
    filter_map_collect(iterable, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_helpers() {
        let doubled: Vec<_> = map_collect([1, 2, 3], |value| value * 2);
        assert_eq!(doubled, vec![2, 4, 6]);

        let filtered: Vec<_> =
            filter_map_collect([1, 2, 3, 4], |value| (value % 2 == 0).then_some(value * 10));
        assert_eq!(filtered, vec![20, 40]);
    }
}

#[cfg(all(test, feature = "alloc"))]
mod alloc_tests {
    use super::*;

    #[test]
    fn test_iter_alloc_helpers() {
        let joined = [1, 2, 3].iter().join_("-");
        assert_eq!(joined, "1-2-3");

        let mapped = map_collect_vec([1, 2, 3], |value| value * 3);
        assert_eq!(mapped, vec![3, 6, 9]);

        let filtered =
            filter_map_collect_vec([1, 2, 3, 4], |value| (value % 2 == 1).then_some(value));
        assert_eq!(filtered, vec![1, 3]);
    }
}
