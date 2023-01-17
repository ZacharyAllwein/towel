use towel::functor::Functor;

#[test]
fn vec_functor() {
    assert_eq!(vec![1, 2].fmap(|x| x.to_string()), vec!["1", "2"]);
}
