use crate::applicative::Applicative;
use crate::functor::Functor;

pub trait Monad<A>: Applicative<A> {
    type MHKT<B>;

    fn ret(a: A) -> Self
    where
        Self: Sized,
    {
        <Self as Applicative<A>>::pure(a)
    }

    fn bind<B, F: Fn(&A) -> Self::MHKT<B>>(&self, f: F) -> Self::MHKT<B>;
}

//basically a flat map for vec
impl<A: 'static> Monad<A> for Vec<A> {
    type MHKT<B> = Vec<B>;

    fn bind<B, F: Fn(&A) -> Self::MHKT<B>>(&self, f: F) -> Self::MHKT<B> {
        self.fmap(f).into_iter().flatten().collect()
    }
}

//map value inside Option to Option then reduce structure
impl<A> Monad<A> for Option<A> {
    type MHKT<B> = Option<B>;

    fn bind<B, F: Fn(&A) -> Self::MHKT<B>>(&self, f: F) -> Self::MHKT<B> {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
