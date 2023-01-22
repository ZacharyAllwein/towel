use towel::traits::Applicative;

fn add_to_string(a: i32, b: i32) -> String{

    (a + b).to_string()

}


#[test]
fn vec_applicative(){

    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];

    assert_eq!(a.lift_a2(b, add_to_string), vec!["2", "3", "4", "3", "4", "5", "4", "5", "6"]);

    
}

#[test]
fn opt_applicative(){

    let a = Some(1);
    let b = Some(2);

    assert_eq!(a.lift_a2(b, add_to_string), Some("3".to_string()));
}
