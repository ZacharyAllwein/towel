use crate::prelude::*;

/// A enum similar to haskells Either type represents the possibility of
/// being of type A ([Left]) or type B ([Right])
/// 
/// # Examples
///
/// Basic Usage:
///
/// ```
/// use towel::data_structures::Either::{self, Left, Right};
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
/// # [Functor]
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}


impl<'a, A: 'a + Clone, B: 'a> Functor<'a, B> for Either<A, B> {
    type HKT<C> = Either<A, C>;

    fn fmap<C, F: Fn(&B) -> C>(&self, f: F) -> Self::HKT<C> {
        match self {
            Left(a) => Left(a.clone()),
            Right(b) => Right(f(b)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Applicative<'a, B> for Either<A, B> {
    type F<C: 'a> = fn(&B) -> C;

    fn pure(a: B) -> Self {
        Right(a)
    }

    fn apply<C: 'a>(&self, other: &Self::HKT<Self::F<C>>) -> Self::HKT<C> {
        match (self, other) {
            (Left(a), _) => Left(a.clone()),
            (_, Left(a)) => Left(a.clone()),
            (Right(a), Right(f)) => Right(f(a)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Monad<'a, B> for Either<A, B> {
    fn bind<C, F: Fn(&B) -> Self::HKT<C>>(&self, f: F) -> Self::HKT<C> {
        match self {
            Left(a) => Left(a.clone()),
            Right(a) => f(a),
        }
    }
}
