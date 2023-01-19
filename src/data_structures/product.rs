use crate::prelude::*;

pub struct Product<A, B>(A, B);

impl<A: Semigroup, B: Semigroup> Semigroup for Product<A, B> {
    fn combine(&self, other: &Self) -> Self {
        Product(
            self.0.combine(&other.0),
            self.1.combine(&other.1),
        )
    }
}

impl<A: Monoid, B: Monoid> Monoid for Product<A, B> {
    fn mempty() -> Self {
        Product(<A as Monoid>::mempty(), <B as Monoid>::mempty())
    }
}

impl<'a, A: 'a + Clone, B: 'a> Functor<'a, B> for Product<A, B> {
    type HKT<C> = Product<A, C>;

    fn fmap<C, F: Fn(&B) -> C>(&self, f: F) -> Self::HKT<C> {
        Product(self.0.clone(), f(&self.1))
    }
}

impl<'a, A: 'a + Clone + Monoid, B: 'a> Applicative<'a, B> for Product<A, B> {
    type F<C: 'a> = fn(&B) -> C;

    fn pure(a: B) -> Self {
        Product(<A as Monoid>::mempty(), a)
    }

    fn app<C: 'a>(&self, other: &Self::HKT<Self::F<C>>) -> Self::HKT<C> {
        Product(
            self.0.combine(&other.0),
            (other.1)(&self.1),
        )
    }
}

impl<'a, A: 'a + Clone + Monoid, B: 'a> Monad<'a, B> for Product<A, B> {
    fn bind<C, F: Fn(&B) -> Self::HKT<C>>(&self, f: F) -> Self::HKT<C> {
        let Product(a2, b2) = f(&self.1);

        Product(self.0.combine(&a2), b2)
    }
}

impl<A, B> Product<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Product(a, b)
    }

    pub fn left(&self) -> &A {
        &self.0
    }

    pub fn right(&self) -> &B {
        &self.1
    }
}
