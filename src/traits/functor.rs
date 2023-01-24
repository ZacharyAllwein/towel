use crate::traits::Bound;

/// Trait for function application abstracted over structure
pub trait Functor<A, B, F: Fn(A) -> B>: Bound<B> {

    ///Lifts a function over a structure.
    ///Changes values inside structure, but not structure itself.
    fn fmap(self, f: F) -> Self::Bound;
}

impl<A, B, F: Fn(A) -> B> Functor<A, B, F> for Vec<A> {
    fn fmap(self, f: F) -> Self::Bound {
        self.into_iter().map(&f).collect()
    }
}

impl<A, B, F: Fn(A) -> B> Functor<A, B, F> for Option<A> {
    fn fmap(self, f: F) -> Self::Bound {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
