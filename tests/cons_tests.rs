use towel::prelude::*;

#[test]
fn vec_cons(){
    let x = vec![];

    assert_eq!(x.clone().layer(3), vec![3]);
    assert_eq!(x.layer(3).next(), (Some(3), vec![]));
}
