use crate::prelude::{Bound, Applicative, Functor, Monad};
use std::marker::PhantomData;


#[derive(Debug, PartialEq)]
pub struct OptionT<M, A>(M, PhantomData<A>);

impl<M, A, B> Bound<B> for OptionT<M, A>
where
    M: Bound<Option<B>>
{
    
    //the Bound for optionT, is optionT around the bound for the inner
    //monad when it has an option in it
    type Bound = OptionT<<M as Bound<Option<B>>>::Bound, PhantomData<B>>;
}

impl<M, A, B, F> Functor<A, B, F> for OptionT<M, A>
where
    M: Functor<Option<A>, Option<B>, Box<dyn Fn(Option<A>) -> Option<B>>>,
    F: Fn(A) -> B,
{
    fn fmap(self, f: F) -> Self::Bound{

      let g = |a: Option<A>| a.fmap(&f);

      OptionT(self.0.fmap(), PhantomData)
    }
}
