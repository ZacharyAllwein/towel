use crate::traits::{Applicative, Bound, Functor};
/// Trait describing function application in a context
/// where funtion application generates more structure

//don't need control of <Self as Applicative>::Other, so just writin it off
pub trait Monad<A, B, F: Fn(A) -> Self::Bound>: Bound<B> {
    fn ret(a: B) -> Self::Bound;

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

//map value inside Option to Option then reduce structure
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
