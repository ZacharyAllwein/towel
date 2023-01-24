
/// A helper trait to encapsulate higher kinded behavior and unify
/// return values of function from Functor, Applicative, and Monad trait
pub trait Bound<B> {
    type Bound;
}

//feels like a nothing burger, but trust me it's awesome
impl<A, B> Bound<B> for Vec<A> {
    type Bound = Vec<B>;
}

impl<A, B> Bound<B> for Option<A> {
    type Bound = Option<B>;
}
