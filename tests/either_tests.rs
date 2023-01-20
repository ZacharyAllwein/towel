use towel::prelude::*;

#[test]
fn sum_functor() {
    let sum1: Either<i32, i32> = Right(1);
    let sum2: Either<i32, i32> = Left(1);
    let f: fn(&i32) -> i32 = |x| x + 1;

    //works
    assert_eq!(sum1.fmap(f), Right(2));

    //does not work
    assert_ne!(sum2.fmap(f), Left(2));
}

#[test]
fn sum_applicative() {
    let sum1: Either<i32, i32> = Right(1);
    let sum2: Either<i32, i32> = Left(1);
    let sumf: Either<i32, fn(&i32) -> i32> = Right(|&x| x + 1);

    //should work
    assert_eq!(sum1.app(&sumf), Right(2));

    //does not work
    assert_ne!(sum2.app(&sumf), Left(2));
}

#[test]
fn sum_monad() {
    let sum1: Either<i32, i32> = Right(1);
    let sum2: Either<i32, i32> = Left(1);
    let f: fn(&i32) -> Either<i32, i32> = |&x| Either::ret(x + 1);

    assert_eq!(sum1.bind(f), Right(2));

    //does not work
    assert_ne!(sum2.bind(f), Left(2));
}
