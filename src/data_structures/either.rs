use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}


impl<'a, A: 'a + Clone, B: 'a> Functor<'a, B> for Either<A, B> {
    type HKT<C> = Either<A, C>;

    fn fmap<C, F: Fn(&B) -> C>(&self, f: F) -> Self::HKT<C> {
        match self {
            Left(a) => Left(a.clone()),
            Right(b) => Right(f(b)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Applicative<'a, B> for Either<A, B> {
    type F<C: 'a> = fn(&B) -> C;

    fn pure(a: B) -> Self {
        Right(a)
    }

    fn app<C: 'a>(&self, other: &Self::HKT<Self::F<C>>) -> Self::HKT<C> {
        match (self, other) {
            (Left(a), _) => Left(a.clone()),
            (_, Left(a)) => Left(a.clone()),
            (Right(a), Right(f)) => Right(f(a)),
        }
    }
}

impl<'a, A: 'a + Clone, B: 'a> Monad<'a, B> for Either<A, B> {
    fn bind<C, F: Fn(&B) -> Self::HKT<C>>(&self, f: F) -> Self::HKT<C> {
        match self {
            Left(a) => Left(a.clone()),
            Right(a) => f(a),
        }
    }
}
