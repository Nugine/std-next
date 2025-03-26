use stdx::assert_not;

#[test]
fn test_assert_false() {
    assert_not!(1 == 2);
}

#[should_panic(expected = "assertion failed: !(1 != 2)")]
#[test]
fn test_assert_true() {
    assert_not!(1 != 2);
}
