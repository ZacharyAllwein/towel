use towel::applicative::Applicative;

#[test]
fn vec_applicative() {
    let f_vec: Vec<&dyn Fn(&i32) -> i32> = vec![&|x| x + 1, &|x| x * 2];

    assert_eq!(vec![1, 2].app(&f_vec), vec![2, 3, 2, 4])
}

#[test]
fn opt_applicative() {
    assert_eq!(
        Some(1).app(&Some(&|x| x.to_string())),
        Some("1".into())
    )
}
