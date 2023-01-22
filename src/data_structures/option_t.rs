use crate::prelude::{Monad, Functor, Either, Left};
use std::marker::PhantomData;


pub struct OptionT<M, A>(M, PhantomData<A>);

impl<'a, M: 'a, A: 'a> Functor<'a, A> for OptionT<M, A>
    where M: Functor<'a, Option<A>>{
    
    type HKT<B> = OptionT<<M as Functor<'a, Option<A>>>::HKT<Option<B>>, B>;

    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::HKT<B>{
        OptionT(self.0.fmap(move |a| a.fmap(&f)), PhantomData)
    }
}

