#[must_use]
pub fn default<T: Default>() -> T {
    T::default()
}

pub fn default_with<T: Default>(f: impl FnOnce(&mut T)) -> T {
    let mut t = T::default();
    f(&mut t);
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_helpers() {
        let default_option: Option<u8> = default();
        assert_eq!(default_option, None);

        let customized = default_with(|data: &mut Vec<u8>| data.extend([1, 2, 3]));
        assert_eq!(customized, vec![1, 2, 3]);
    }
}
