#[macro_export]
macro_rules! cfg_group {
    ($($item:item)*) => {
        $($item)*
    }
}

#[macro_export]
macro_rules! assert_not {
    ($expr:expr) => {
        assert!(!$expr);
    };
}
