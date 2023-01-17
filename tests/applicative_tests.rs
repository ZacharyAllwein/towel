use towel::applicative::Applicative;

fn add_one(x: &i32) -> i32{
    *x + 1
}

#[test]
fn vec_applicative(){
    let f_vec = vec![&add_one];

    assert_eq!(vec![1, 2].app(&f_vec),
               vec![2, 3])
}
