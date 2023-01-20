use crate::prelude::*;


/// Newtype wrapper for tuple, more exercise than useful
pub struct And<A, B>(A, B);

impl<A: Semigroup, B: Semigroup> Semigroup for And<A, B> {
    fn combine(&self, other: &Self) -> Self {
        And(
            self.0.combine(&other.0),
            self.1.combine(&other.1),
        )
    }
}

impl<A: Monoid, B: Monoid> Monoid for And<A, B> {
    fn empty() -> Self {
        And(<A as Monoid>::empty(), <B as Monoid>::empty())
    }
}

impl<'a, A: 'a + Clone, B: 'a> Functor<'a, B> for And<A, B> {
    type HKT<C> = And<A, C>;

    fn fmap<C, F: Fn(&B) -> C>(&self, f: F) -> Self::HKT<C> {
        And(self.0.clone(), f(&self.1))
    }
}

impl<'a, A: 'a + Clone + Monoid, B: 'a> Applicative<'a, B> for And<A, B> {
    type F<C: 'a> = fn(&B) -> C;

    fn pure(a: B) -> Self {
        And(<A as Monoid>::empty(), a)
    }

    fn apply<C: 'a>(&self, other: &Self::HKT<Self::F<C>>) -> Self::HKT<C> {
        And(
            self.0.combine(&other.0),
            (other.1)(&self.1),
        )
    }
}

impl<'a, A: 'a + Clone + Monoid, B: 'a> Monad<'a, B> for And<A, B> {
    fn bind<C, F: Fn(&B) -> Self::HKT<C>>(&self, f: F) -> Self::HKT<C> {
        let And(a2, b2) = f(&self.1);

        And(self.0.combine(&a2), b2)
    }
}

impl<A, B> And<A, B> {
    pub fn new(a: A, b: B) -> Self {
        And(a, b)
    }

    pub fn left(&self) -> &A {
        &self.0
    }

    pub fn right(&self) -> &B {
        &self.1
    }
}
