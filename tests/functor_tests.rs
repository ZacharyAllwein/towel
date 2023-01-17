use towel::functor::Functor;

#[test]
fn vec_functor() {
    assert_eq!(vec![1, 2].fmap(|x| x.to_string()), vec!["1", "2"]);
}

#[test]
fn opt_functor() {
    assert_eq!(Some(1).fmap(|x| x.to_string()), Some("1".into()));
}
