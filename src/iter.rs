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
