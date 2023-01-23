use towel::traits::Monad;

#[test]
fn vec_monad() {
    let a: Vec<i32> = vec![1, 2, 3];
    let f: fn(i32) -> Vec<String> = |a| vec![a.to_string(), a.to_string()];

    assert_eq!(a.bind(f), vec!["1", "1", "2", "2", "3", "3"]);
}

#[test]
fn opt_monad() {
    let a = Some(1);

    let f: fn(i32) -> Option<String> = |a| {
        if a < 10 {
            Some((a + 1).to_string())
        } else {
            None
        }
    };

    assert_eq!(a.bind(f), Some("2".to_string()))
}
