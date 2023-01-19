use towel::prelude::*;
use towel::sum::Sum;

#[test]
fn sum_functor(){
  //only works on b variant
  assert_eq!(Sum::<i32, i32>::B(1).fmap(|x| x + 1), Sum::B(2));
  assert_eq!(Sum::<i32, i32>::A(1).fmap(|x| x + 1), Sum::A(1));
}

#[test]
fn sum_applicative(){
    let sum1: Sum<i32, i32> = Sum::A(1);
    let sum2: Sum<i32, i32> = Sum::B(1);
    let sumf: Sum<i32, fn(&i32) -> i32> = Sum::B(|&x| x + 1);
    
    //no effect can only satisfy Sum<A, C> by returning A
    assert_eq!(sum1.app(&sumf), sum1);

    //should work
    assert_eq!(sum2.app(&sumf), Sum::<i32, i32>::B(2));
}

