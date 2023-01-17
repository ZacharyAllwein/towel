use towel::monoid::Monoid;

#[test]
fn vec_mempty() {
    assert_eq!(<Vec<i32> as Monoid>::mempty(), vec![]);
}

#[test]
fn opt_mempty() {
    assert_eq!(<Option<Vec<i32>> as Monoid>::mempty(), None);
}
