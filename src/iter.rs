#[cfg(feature = "alloc")]
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::String;

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
