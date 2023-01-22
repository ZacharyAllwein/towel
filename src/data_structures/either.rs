use crate::prelude::*;

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
///
/// # Functor
///
/// Fmapping a Right works as expected producing a
/// new Right with the fmapped value. However it is idempotent over
/// Left due to the nature of the [Functor] trait.
///
/// ```
/// # use towel::prelude::*;
///
/// let l: Either<i32, i32> = Left(2);
/// let r: Either<i32, i32> = Right(2);
///
/// //idempotent
/// assert_eq!(l.fmap(|x| x + 1), Left(2));
///
/// //as expected
/// assert_eq!(r.fmap(|x| x + 1), Right(3));
/// ```
///
/// # Applicative
///
/// Like [Functor] there are similar constraints over the [Applicative]
/// for Either. Applying over a Left is idempotent.
/// Only a Right applied to a Right has function application behavior.
///
/// ```
/// # use towel::prelude::*;
///
/// let l: Either<i32, i32> = Left(2);
/// let r: Either<i32, i32> = Right(2);
///
/// let rf: Either<i32, fn(&i32) -> String> = Right(|&x| x.to_string());
///
/// //idempotent
/// assert_eq!(l.apply(&rf), Left::<i32, String>(2));
///
/// //as expected
/// assert_eq!(r.apply(&rf), Right("2".to_string()));
///```
///
/// # Monad
/// [Binding](Monad) over a Left has no effect, but works over a Right.
///
/// ```
/// # use towel::prelude::*;
///
/// let l: Either<&str, &str> = Left("hello there");
/// let r: Either<&str, &str> = Right("32");
///
/// let f: fn(&&str) -> Either<&'static str, i32> = |&x| {
///     match x.parse::<i32>(){
///         Ok(x) => Right(x),
///         Err(_) => Left("parse failed")
///     }
/// };
///
/// //no effect
/// assert_eq!(l.bind(f), Left::<&str, i32>("hello there"));
///
/// //as expected
/// assert_eq!(r.bind(f), Right(32));
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
