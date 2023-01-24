use crate::traits::{Applicative, Bound, Functor};

/// Trait describing function application in a context
/// where funtion application generates more structure
///
/// # Foreign Type Impl Examples
///
/// ## Vector
///
/// return for Vec lifts one value into a Vector
///
/// ```
/// # use towel::traits::{Monad, Bound};
///
/// assert_eq!(<Vec<i32> as Monad<i32, i32, fn(i32) -> <Vec<i32> as Bound<i32>>::Bound>>::ret(1), vec![1]);
/// ```
///
/// bind for Vec is same as flatmap, maps each value to Vec and flattens
///
/// ```
/// # use towel::traits::Monad;
///
/// let v = vec![1, 2];
///
/// assert_eq!(v.bind(|x| vec![x, x]), vec![1, 1, 2, 2]);
/// ```
///
/// ## Option
///
/// return for option lifts value into Some
///
/// ```
/// # use towel::traits::{Monad, Bound};
///
/// assert_eq!(<Option<i32> as Monad<i32, i32, fn(i32) -> <Option<i32> as Bound<i32>>::Bound>>::ret(1), Some(1));
/// ```
///
/// bind for Option allows for application of a fn that could result in None while
/// not creating additional structure
///
/// ```
/// # use towel::traits::Monad;
///
/// assert_eq!(Some(1).bind(|x| Some(x + 1)), Some(2));
/// assert_eq!(Some(1).bind(|_| None::<i32>), None);
/// ```
pub trait Monad<A, B, F: Fn(A) -> Self::Bound>: Bound<B> {
    /// Lifts value into Monadic context
    fn ret(a: B) -> Self::Bound;

    /// Applies a function to value(s) inside Monad that generate extra
    /// structure of the same type, and reduces that structure
    fn bind(self, f: F) -> Self::Bound;
}

impl<A: Clone, B, F: Fn(A) -> Self::Bound> Monad<A, B, F> for Vec<A> {
    fn ret(a: B) -> Self::Bound {
        <Self as Applicative<A, A, B, fn(A, A) -> B>>::pure(a)
    }

    fn bind(self, f: F) -> Self::Bound {
        self.fmap(f).into_iter().flatten().collect()
    }
}

impl<A, B, F: Fn(A) -> Self::Bound> Monad<A, B, F> for Option<A> {
    fn ret(a: B) -> Self::Bound {
        <Self as Applicative<A, A, B, fn(A, A) -> B>>::pure(a)
    }

    fn bind(self, f: F) -> Self::Bound {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
