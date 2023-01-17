use towel::applicative::Applicative;
use towel::functor::Functor;

#[test]
fn vec_applicative() {
    let f_vec: Vec<&dyn Fn(&String) -> String> =
        vec![&|x| format!("{}{}", x, 2), &|x| format!("{}{}", x, 1)];

    assert_eq!(
        vec!["hello".into(), "world".into()].app(&f_vec),
        vec!["hello2", "world2", "hello1", "world1"].fmap(|s| s.to_string())
    );
    assert_eq!(<Vec<i32> as Applicative<i32>>::pure(1), vec![1]);
}

#[test]
fn opt_applicative() {
    assert_eq!(Some(1).app(&Some(|x| x.to_string())), Some("1".into()));
    assert_eq!(<Option<i32> as Applicative<i32>>::pure(1), Some(1));
}
