#![allow(unsafe_code)]

use crate::sealed::Sealed;

pub trait SliceExt<T>: Sealed {
    fn as_chunks_<const N: usize>(&self) -> (&[[T; N]], &[T]);

    fn as_chunks_mut_<const N: usize>(&mut self) -> (&mut [[T; N]], &mut [T]);
}

impl<T> Sealed for [T] {}

impl<T> SliceExt<T> for [T] {
    fn as_chunks_<const N: usize>(&self) -> (&[[T; N]], &[T]) {
        assert!(N > 0, "chunk size must be non-zero");
        let base = self.as_ptr();
        let len = self.len();
        let (div, rem) = (len / N, len % N);
        let chunks = unsafe { slice(base.cast(), div) };
        let rest = unsafe { slice(base.add(div * N), rem) };
        (chunks, rest)
    }

    fn as_chunks_mut_<const N: usize>(&mut self) -> (&mut [[T; N]], &mut [T]) {
        assert!(N > 0, "chunk size must be non-zero");
        let base = self.as_mut_ptr();
        let len = self.len();
        let (div, rem) = (len / N, len % N);
        let chunks = unsafe { slice_mut(base.cast(), div) };
        let rest = unsafe { slice_mut(base.add(div * N), rem) };
        (chunks, rest)
    }
}

unsafe fn slice<'a, T>(data: *const T, len: usize) -> &'a [T] {
    core::slice::from_raw_parts(data, len)
}

unsafe fn slice_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    core::slice::from_raw_parts_mut(data, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_chunks() {
        let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (chunks, rest) = data.as_chunks_::<3>();
        assert_eq!(chunks, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(rest, [10]);

        let (chunks, rest) = data.as_chunks_::<4>();
        assert_eq!(chunks, [[1, 2, 3, 4], [5, 6, 7, 8]]);
        assert_eq!(rest, [9, 10]);

        let (chunks, rest) = data.as_chunks_::<10>();
        assert_eq!(chunks, [[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
        assert_eq!(rest, &[]);

        let (chunks, rest) = data.as_chunks_mut_::<3>();
        assert_eq!(chunks, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(rest, [10]);

        let (chunks, rest) = data.as_chunks_mut_::<4>();
        assert_eq!(chunks, [[1, 2, 3, 4], [5, 6, 7, 8]]);
        assert_eq!(rest, [9, 10]);

        let (chunks, rest) = data.as_chunks_mut_::<10>();
        assert_eq!(chunks, [[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
        assert_eq!(rest, &[]);
    }
}
