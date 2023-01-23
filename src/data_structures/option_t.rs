use crate::prelude::{Applicative, Functor, Monad};
use std::marker::PhantomData;


#[derive(Debug, PartialEq)]
pub struct OptionT<M, A>(M, PhantomData<A>);

//just reducing visual noise
type O<A> = Option<A>;

//helper type for wrapped Functor
type InnerMapped<M, A, B, F> = <M as Functor<O<A>, O<B>, F>>::Mapped;

//helper type for wrapped Applicative
type InnerOther<M, A, B, C, F, G> =
  <M as Applicative<O<A>, O<B>, O<C>, F, G>>::Other;


impl<M, A, B, F> Functor<A, B, F> for OptionT<M, A>
where
    F: Fn(A) -> B,
    M: Functor<O<A>, O<B>, F>,
{
    type Mapped = OptionT<InnerMapped<M, A, B>, B>;

    fn fmap(self, f: F) -> Self::Mapped {
        OptionT(self.0.fmap(move |a| a.fmap(&f)), PhantomData)
    }
}

impl<M, A, B, C, F, G> Applicative<A, B, C, F, G> for OptionT<M, A>
where
    F: Fn(A, B) -> C,
    G: Fn(A) -> C,
    M: Applicative<O<A>, O<B>, O<C>, F, G>,
{
    type Other = OptionT<InnerOther<M, A, B, C, F, G>, B>;

    fn pure(a: C) -> Self::Mapped {
        OptionT(
            <M as Applicative<O<A>, O<B>, O<C>>>::pure(Some(a)),
            PhantomData,
        )
    }


    fn lift_a2(self, other: Self::Other, f: F) -> Self::Mapped{
        OptionT(self.0.lift_a2(other.0, move |a, b| a.lift_a2(b, &f)), PhantomData)
    }
}


impl<M, A, B, F, G, H> Monad<A, B, F, G, H> for OptionT<M, A>
where F: Fn(A) -> Self::Mapped,
      G: Fn(A, A) -> B,
      H: Fn(A) -> B,
      M: Monad<O<A>, O<B>>{

    fn bind(self, f: F) -> Self::Mapped{
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
