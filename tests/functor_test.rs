use towel::traits::Functor;

fn pluse_one_string(a: i32) -> String{
    (a + 1).to_string()
}

#[test]
fn vec_functor(){
    let x = vec![1, 2, 3];

    assert_eq!(x.fmap(pluse_one_string), vec!["2", "3", "4"]);
}

#[test]
fn opt_functor(){
    let x = Some(1);
    
    assert_eq!(x.fmap(pluse_one_string), Some("2".to_string()));
}
