use crate::prelude::{Applicative, Functor, Monad};
use std::marker::PhantomData;

pub struct OptionT<M, A>(M, PhantomData<A>);

impl<'a, M: 'a, A: 'a> Functor<'a, A> for OptionT<M, A>
where
    M: Functor<'a, Option<A>>,
{
    type HKT<B: 'a> = OptionT<<M as Functor<'a, Option<A>>>::HKT<Option<B>>, B>;

    fn fmap<B: 'a, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::HKT<B> {
        OptionT(self.0.fmap(move |a| a.fmap(&f)), PhantomData)
    }
}

impl<'a, M: 'a, A: 'a> Applicative<'a, A> for OptionT<M, A>
where
    M: Applicative<'a, Option<A>>,
{

    fn pure(a: A) -> Self {
        OptionT(
            <M as Applicative<Option<A>>>::pure(<Option<A> as Applicative<A>>::pure(a)),
            PhantomData,
        )
    }


    fn lift_a2<B: 'a, C: 'a, F: Fn(&A, &B) -> C + 'a>(&'a self, other: &'a Self::HKT<B>, f: F) -> Self::HKT<C>{
        OptionT(self.0.lift_a2(&other.0, move |a, b| a.lift_a2(b, &f)), PhantomData)
    }
}
