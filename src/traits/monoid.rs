use crate::traits::Semigroup;

//a monoid is a associative binary operation over a set with an identity value
pub trait Monoid: Semigroup {
    fn mappend(&self, other: &Self) -> Self {
        self.combine(&other)
    }
    fn mempty() -> Self;
}

impl<A: Clone> Monoid for Vec<A> {
    fn mempty() -> Self {
        vec![]
    }
}

impl<A: Semigroup + Clone> Monoid for Option<A> {
    fn mempty() -> Self {
        None
    }
}
