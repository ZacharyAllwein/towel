use towel::monad::Monad;

#[test]
fn vec_monad() {
    assert_eq!(vec![1, 2].bind(|&x| vec![x, x]), vec![1, 1, 2, 2]);
}

#[test]
fn opt_monad() {
    assert_eq!(Some(1).bind(|x| Some(x + 1)), Some(2));
}
