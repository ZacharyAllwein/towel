use crate::traits::applicative::Applicative;
use crate::traits::functor::Functor;

pub trait Monad<'a, A>: Applicative<'a, A> {
    fn ret(a: A) -> Self
    where
        Self: Sized,
    {
        <Self as Applicative<A>>::pure(a)
    }

    fn bind<B, F: Fn(&A) -> Self::HKT<B> + 'a>(&'a self, f: F) -> Self::HKT<B>;
}

//basically a flat map for vec
impl<'a, A: 'a> Monad<'a, A> for Vec<A> {
    fn bind<B, F: Fn(&A) -> Self::HKT<B>>(&self, f: F) -> Self::HKT<B> {
        self.fmap(f).into_iter().flatten().collect()
    }
}

//map value inside Option to Option then reduce structure
impl<'a, A: 'a> Monad<'a, A> for Option<A> {
    fn bind<B, F: Fn(&A) -> Self::HKT<B>>(&self, f: F) -> Self::HKT<B> {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
