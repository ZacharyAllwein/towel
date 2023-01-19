use towel::state::State;
use towel::functor::Functor;

#[test]
fn state_functor(){
    
    let s1 = State::new(Box::new(|_| ((), 1)));

    //s1 is now borrowed;
    let s2 = s1.fmap(|_| 1);

    assert_eq!(s2.eval(0), (1, 1))
}
