use crate::traits::{Bound, Functor};

/// Trait describing function application in a context
/// where funtion application generates more structure
pub trait Monad<A, B, F: Fn(A) -> Self::Bound>: Bound<B> {
    /// Applies a function to value(s) inside Monad that generate extra
    /// structure of the same type, and reduces that structure
    ///
    /// ```
    /// # use towel::traits::Monad;
    /// let v = vec![1, 2];
    ///
    /// //closure produces another vec, but the result is flattened
    /// assert_eq!(v.bind(|x| vec![x, x]), vec![1, 1, 2, 2]);
    fn bind(self, f: F) -> Self::Bound;
}

impl<A: Clone, B, F: Fn(A) -> Self::Bound> Monad<A, B, F> for Vec<A> {
    fn bind(self, f: F) -> Self::Bound {
        self.fmap(f).into_iter().flatten().collect()
    }
}

impl<A, B, F: Fn(A) -> Self::Bound> Monad<A, B, F> for Option<A> {
    fn bind(self, f: F) -> Self::Bound {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
