use crate::sealed::Sealed;

use alloc::vec::Vec;

pub trait VecExt<T>: Sealed {
    fn remove_if<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool;
}

impl<T> Sealed for Vec<T> {}

impl<T> VecExt<T> for Vec<T> {
    fn remove_if<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.retain_mut(|x| !f(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_if() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.remove_if(|x| *x % 2 == 0);
        assert_eq!(vec, [1, 3, 5]);
    }
}
