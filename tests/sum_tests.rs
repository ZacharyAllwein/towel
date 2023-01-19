use towel::data_structures::Sum;
use towel::prelude::*;

#[test]
fn sum_functor() {
    let sum1: Sum<i32, i32> = Sum::B(1);
    let sum2: Sum<i32, i32> = Sum::A(1);
    let f: fn(&i32) -> i32 = |x| x + 1;

    //works
    assert_eq!(sum1.fmap(f), Sum::B(2));

    //does not work
    assert_ne!(sum2.fmap(f), Sum::A(2));
}

#[test]
fn sum_applicative() {
    let sum1: Sum<i32, i32> = Sum::B(1);
    let sum2: Sum<i32, i32> = Sum::A(1);
    let sumf: Sum<i32, fn(&i32) -> i32> = Sum::B(|&x| x + 1);

    //should work
    assert_eq!(sum1.app(&sumf), Sum::B(2));

    //does not work
    assert_ne!(sum2.app(&sumf), Sum::A(2));
}

#[test]
fn sum_monad() {
    let sum1: Sum<i32, i32> = Sum::B(1);
    let sum2: Sum<i32, i32> = Sum::A(1);
    let f: fn(&i32) -> Sum<i32, i32> = |&x| Sum::ret(x + 1);

    assert_eq!(sum1.bind(f), Sum::B(2));

    //does not work
    assert_ne!(sum2.bind(f), Sum::A(2));
}
