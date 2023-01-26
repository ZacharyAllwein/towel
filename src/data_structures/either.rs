use crate::prelude::{identity, Applicative, Bound, Functor, Left, Monad, Right};

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

impl<A, B, C> Bound<C> for Either<A, B> {
    type Bound = Either<A, C>;

    fn wrap(a: C) -> Self::Bound {
        Right(a)
    }
}

impl<A, B, C, F: FnOnce(B) -> C> Functor<B, C, F> for Either<A, B> {
    fn fmap(self, f: F) -> Self::Bound {
        match self {
            Left(a) => Left(a),
            Right(b) => Right(f(b)),
        }
    }
}

impl<A, B, C, D, F: FnOnce(B, C) -> D> Applicative<B, C, D, F> for Either<A, B> {
    type Other = Either<A, C>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        match (self, other) {
            (Left(a), _) => Left(a),
            (_, Left(a)) => Left(a),
            (Right(a), Right(b)) => Right(f(a, b)),
        }
    }
}

impl<A, B, C, F: FnOnce(B) -> Self::Bound> Monad<B, C, F> for Either<A, B> {
    fn bind(self, f: F) -> Self::Bound {
        match self {
            Left(a) => Left(a),
            Right(a) => f(a),
        }
    }
}

impl<A, B> Either<A, B> {
    /// Idiomatic map for either. Maps value inside a Right variant of Either,
    /// leaving Lefts unchanged
    ///
    /// ```
    /// # use towel::data_structures::Either::{self, Left, Right};
    /// let f = |x: i32| (x + 1).to_string();
    ///
    /// assert_eq!(Right::<Vec<i32>, i32>(3).map(f), Right("4".to_string()));
    /// assert_eq!(Left::<Vec<i32>, i32>(vec![2]).map(f), Left(vec![2]));
    //just wrapping fmap so data structure can be used idiomatically
    pub fn map<F: FnOnce(B) -> C, C>(self, f: F) -> Either<A, C> {
        self.fmap(f)
    }

    /// Takes two fns and applies based on Either variant of self
    ///
    /// ```
    /// # use towel::data_structures::Either::{self, Left, Right};
    /// let l = Left("hello");;
    /// let r = Right("world");
    ///
    /// let f: fn(Either<&'static str, &'static str>) -> Option<&'static str> =
    ///     |x| x.either(|a| Some(a), |_| None);
    ///
    /// assert_eq!(f(l), Some("hello"));
    /// assert_eq!(f(r), None);
    pub fn either<F, G, C>(self, f: F, g: G) -> C
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> C,
    {
        match self {
            Left(a) => f(a),
            Right(b) => g(b),
        }
    }

    /// Allows a choice between two values of the same type
    /// to be made based on whether an Either is Left or Right.
    /// Ignores data inside variant.
    ///
    /// ```
    /// # use towel::data_structures::Either::{self, Left, Right};
    /// let l = Left(10);
    /// let r = Right(20);
    ///
    /// let f: fn(Either<i32, i32>) -> &'static str = |x| x.choice("foo", "bar");
    ///
    /// assert_eq!(f(l), "foo");
    /// assert_eq!(f(r), "bar");
    pub fn choice<C>(&self, a: C, b: C) -> C {
        match self {
            Left(_) => a,
            Right(_) => b,
        }
    }

    /// Get value out of left or return default value
    ///
    /// ```
    /// # use towel::data_structures::Either::{Left, Right};
    /// //returns value out of left
    /// assert_eq!(Left::<i32, &'static str>(3).from_left(2), 3);
    ///
    /// //uses default value provided
    /// assert_eq!(Right::<i32, &'static str>("hello").from_left(2), 2);
    pub fn from_left(self, a: A) -> A {
        self.either(identity, move |_| a)
    }

    /// Get value out of right or return default value
    ///
    /// ```
    /// # use towel::data_structures::Either::{Left, Right};
    /// //returns value out of right
    /// assert_eq!(Right::<i32, &'static str>("hello").from_right("world"), "hello");
    ///
    /// //uses default value provided
    /// assert_eq!(Left::<i32, &'static str>(3).from_right("world"), "world");
    pub fn from_right(self, a: B) -> B {
        self.either(move |_| a, identity)
    }

    /// Return true if self is Left variant else false
    pub fn is_left(&self) -> bool {
        self.choice(true, false)
    }

    /// Return true if self is Right variant else false
    pub fn is_right(&self) -> bool {
        self.choice(false, true)
    }
}
