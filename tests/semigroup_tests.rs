use towel::prelude::*;

#[test]
fn vec_combine() {
    assert_eq!(vec![1, 2].combine(&vec![3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn opt_combine() {
    assert_eq!(
        Some(vec![1, 2]).combine(&Some(vec![3, 4])),
        Some(vec![1, 2, 3, 4])
    );
}
