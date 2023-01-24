use crate::traits::{Applicative, Bound, Functor};

/// Trait describing function application in a context
/// where funtion application generates more structure
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
