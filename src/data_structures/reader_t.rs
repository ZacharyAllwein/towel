use crate::traits::{Applicative, Bound, Functor, Monad};
use std::marker::PhantomData;

type RFn<'a, R, M> = Box<dyn FnOnce(R) -> M + 'a>;

/// A reader transformer containing a r -> m a
pub struct ReaderT<'a, R, M, A>(RFn<'a, R, M>, PhantomData<A>);

//Improving clarity of code in impl body
type AsBound<M, B> = <M as Bound<B>>::Bound;

impl<'a, R, M, A, B: 'a> Bound<B> for ReaderT<'a, R, M, A>
where
    M: Bound<B>,
{
    type Bound = ReaderT<'a, R, AsBound<M, B>, PhantomData<B>>;

    fn wrap(a: B) -> Self::Bound {
        //wrap up internal value in monad and return
        //as a reader (r -> m a)
        ReaderT(Box::new(move |_| <M as Bound<B>>::wrap(a)), PhantomData)
    }
}

impl<'a, R, M, A, B, F> Functor<A, B, F> for ReaderT<'a, R, M, A>
where
    R: 'a,
    M: 'a + Functor<A, B, F>,
    A: 'a,
    B: 'a,
    F: 'a + FnOnce(A) -> B,
{
    fn fmap(self, f: F) -> Self::Bound {
        //eval self then map internal value with f
        ReaderT(Box::new(move |r| self.eval(r).fmap(f)), PhantomData)
    }
}

//Improving clarity of code in impl body
type AsOther<M, A, B, C, F> = <M as Applicative<A, B, C, F>>::Other;

impl<'a, R, M, A, B, C, F> Applicative<A, B, C, F> for ReaderT<'a, R, M, A>
where
    R: 'a + Clone,
    M: 'a + Applicative<A, B, C, F>,
    A: 'a,
    B: 'a,
    C: 'a,
    F: 'a + FnOnce(A, B) -> C,
{
    //use the other type on the internal applicative to
    //create other type for ReaderT
    type Other = ReaderT<'a, R, AsOther<M, A, B, C, F>, PhantomData<B>>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        ReaderT(
            Box::new(move |r| {
                //pass arg to self and other and combine results
                //with provided f
                let a = self.eval(r.clone());
                let b = other.eval(r);

                a.lift_a2(b, f)
            }),
            PhantomData,
        )
    }
}

impl<'a, R, M, A, B, F> Monad<A, B, F> for ReaderT<'a, R, M, A>
where
    R: 'a + Clone,
    M: 'a + Monad<A, B, Box<dyn FnOnce(A) -> AsBound<M, B> + 'a>>,
    A: 'a,
    B: 'a,
    F: 'a + FnOnce(A) -> Self::Bound,
{
    fn bind(self, f: F) -> Self::Bound {
        ReaderT(
            //eval self then bind resulted monad with f
            //and return m b by evaling the returned ReaderT
            Box::new(move |r| self.eval(r.clone()).bind(Box::new(move |a| f(a).eval(r)))),
            PhantomData,
        )
    }
}

impl<'a, R, M, A> ReaderT<'a, R, M, A> {
    /// Evalueate the contained fn by passing in an environment: R
    pub fn eval(self, r: R) -> M {
        (self.0)(r)
    }
}
