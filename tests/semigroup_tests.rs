use towel::semigroup::Semigroup;

#[test]
fn vec_combine() {
    assert_eq!(vec![1, 2].combine(&vec![3, 4]), vec![1, 2, 3, 4]);
}
