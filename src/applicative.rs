use crate::functor::Functor;

pub trait Applicative<A>: Functor<A>{
    type AHKT<B>;

    fn pure<B>(a: B) -> Self::AHKT<B>;

    fn app<B, F: Fn(&A) -> B>
          (&self, other: &Self::AHKT<F>) -> Self::AHKT<B>;
}


//ToDo: Make this work with vecs of more than one function
impl<A: Clone> Applicative<A> for Vec<A>{
    type AHKT<B> = Vec<B>;

    fn pure<B>(a: B) -> Self::AHKT<B>{
        vec![a]
    }

    fn app<B, F: Fn(&A) -> B>
          (&self, other: &Self::AHKT<F>) -> Self::AHKT<B>{
        (other.fmap(|f| self.fmap(f))).into_iter()
                                      .flatten()
                                      .collect()

    }
}


