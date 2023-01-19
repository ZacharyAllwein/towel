use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Sum<A, B> {
    A(A),
    B(B),
}


impl<'a, A: 'a + Clone, B: 'a> Functor<'a, B> for Sum<A, B> {
    type HKT<C> = Sum<A, C>;

    fn fmap<C, F: Fn(&B) -> C>(&self, f: F) -> Self::HKT<C> {
        match self {
            Sum::A(a) => Sum::A(a.clone()),
            Sum::B(b) => Sum::B(f(b)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Applicative<'a, B> for Sum<A, B> {
    type F<C: 'a> = fn(&B) -> C;

    fn pure(a: B) -> Self {
        Sum::B(a)
    }

    fn app<C: 'a>(&self, other: &Self::HKT<Self::F<C>>) -> Self::HKT<C> {
        match (self, other) {
            (Sum::A(a), _) => Sum::A(a.clone()),
            (_, Sum::A(a)) => Sum::A(a.clone()),
            (Sum::B(a), Sum::B(f)) => Sum::B(f(a)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Monad<'a, B> for Sum<A, B> {
    fn bind<C, F: Fn(&B) -> Self::HKT<C>>(&self, f: F) -> Self::HKT<C> {
        match self {
            Sum::A(a) => Sum::A(a.clone()),
            Sum::B(a) => f(a),
        }
    }
}
