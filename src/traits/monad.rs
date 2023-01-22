use crate::traits::{Applicative, Functor};

/// Trait describing function application in a context
/// where funtion application generates more structure
//don't need control of <Self as Applicative>::Other, so just writin it off
pub trait Monad<A, B>: Applicative<A, A, B> {

    fn bind<F: Fn(A) -> Self::Mapped>(self, f: F) -> Self::Mapped;
}

impl<A: Clone, B> Monad<A, B> for Vec<A> {
    fn bind<F: Fn(A) -> Self::Mapped>(self, f: F) -> Self::Mapped {
        self.fmap(f).into_iter().flatten().collect()
    }
}

//map value inside Option to Option then reduce structure
impl<A, B> Monad<A, B> for Option<A> {
    fn bind<F: Fn(A) -> Self::Mapped>(self, f: F) -> Self::Mapped {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
