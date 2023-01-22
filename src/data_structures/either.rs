use crate::prelude::{Applicative, Monad, Functor, Left, Right};

/// A enum similar to haskells Either type represents the possibility of
/// being of type A ([Left]) or type B ([Right])
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// # use towel::prelude::*;
///
/// //if a < 10 return a
/// //else return a as a String
/// fn less_than_ten(a: i32) -> Either<String, i32>{
///     if a < 10 {
///         Right(a)
///     }
///     else {
///         Left(a.to_string())
///     }
/// }
///
/// assert_eq!(less_than_ten(9), Right(9));
///
/// assert_eq!(less_than_ten(11), Left("11".to_string()));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B, C> Functor<B, C> for Either<A, B> {
    type Mapped = Either<A, C>;

    fn fmap<F: Fn(B) -> C>(self, f: F) -> Self::Mapped {
        match self {
            Left(a) => Left(a),
            Right(b) => Right(f(b)),
        }
    }
}

impl<A, B, C, D> Applicative<B, C, D> for Either<A, B> {

    type Other = Either<A, C>;

    fn pure(a: D) -> Self::Mapped {
        Right(a)
    }

    fn lift_a2<F: Fn(B, C) -> D>(self, other: Self::Other, f: F) -> Self::Mapped{
        match (self, other) {
            (Left(a), _) => Left(a),
            (_, Left(a)) => Left(a),
            (Right(a), Right(b)) => Right(f(a, b)),
        }
    }
}

impl<A, B, C> Monad<B, C> for Either<A, B> {
    fn bind<F: Fn(B) -> Self::Mapped>(self, f: F) -> Self::Mapped {
        match self {
            Left(a) => Left(a),
            Right(a) => f(a),
        }
    }
}
