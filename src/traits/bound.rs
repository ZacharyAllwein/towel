
/// A helper trait to encapsulate higher kinded behavior and unify
/// return values of function from Functor, Applicative, and Monad trait
pub trait Bound<B> {
    type Bound;

    fn wrap(a: B) -> Self::Bound;
}

impl<A, B> Bound<B> for Vec<A> {
    type Bound = Vec<B>;

    fn wrap(a: B) -> Self::Bound{
        vec![a]
    }
}

impl<A, B> Bound<B> for Option<A> {
    type Bound = Option<B>;

    fn wrap(a: B) -> Self::Bound {
        Some(a)
    }
}
