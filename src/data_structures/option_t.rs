use crate::prelude::{Applicative, Functor, Monad};
use std::marker::PhantomData;


#[derive(Debug, PartialEq)]
pub struct OptionT<M, A>(M, PhantomData<A>);

//just reducing visual noise
type O<A> = Option<A>;

//helper type for wrapped Functor
type InnerMapped<M, A, B> = <M as Functor<O<A>, O<B>>>::Mapped;

//helper type for wrapped Applicative
type InnerOther<M, A, B, C> =
  <M as Applicative<O<A>, O<B>, O<C>>>::Other;


impl<M, A, B> Functor<A, B> for OptionT<M, A>
where
    M: Functor<O<A>, O<B>>,
{
    type Mapped = OptionT<InnerMapped<M, A, B>, B>;

    fn fmap<F: Fn(A) -> B>(self, f: F) -> Self::Mapped {
        OptionT(self.0.fmap(move |a| a.fmap(&f)), PhantomData)
    }
}

impl<M, A, B, C> Applicative<A, B, C> for OptionT<M, A>
where
    M: Applicative<O<A>, O<B>, O<C>>,
{
    type Other = OptionT<InnerOther<M, A, B, C>, B>;

    fn pure(a: C) -> Self::Mapped {
        OptionT(
            <M as Applicative<O<A>, O<B>, O<C>>>::pure(Some(a)),
            PhantomData,
        )
    }


    fn lift_a2<F: Fn(A, B) -> C>(self, other: Self::Other, f: F) -> Self::Mapped{
        OptionT(self.0.lift_a2(other.0, move |a, b| a.lift_a2(b, &f)), PhantomData)
    }
}


impl<M, A, B> Monad<A, B> for OptionT<M, A>
    where M: Monad<O<A>, O<B>>{

    fn bind<F: Fn(A) -> Self::Mapped>(self, f: F) -> Self::Mapped{
        OptionT(self.0.bind(move |a| {
            match a.fmap(&f){
                Some(x) => x.0,
                None => <M as Applicative<O<A>, O<A>, O<B>>>::pure(None),
            }
        }), PhantomData)
    }
}

impl<M, A> OptionT<M, A>
where M: Monad<O<A>, O<A>>{
    pub fn new(a: A) -> <Self as Functor<A, A>>::Mapped{
      <Self as Applicative<A, A, A>>::pure(a) 
    }

    pub fn new_manual(a: M) -> Self{
      OptionT(a, PhantomData)
    }
}
