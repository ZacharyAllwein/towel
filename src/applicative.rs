use crate::functor::Functor;

pub trait Applicative<A>: Functor<A> {
    type AHKT<B>;

    fn pure<B>(a: B) -> Self::AHKT<B>;

    //ToDo: Allow static or dynamic dispatch to be chosen at Applicative instantiation
    fn app<B>(&self, other: &Self::AHKT<&dyn Fn(&A) -> B>) -> Self::AHKT<B>;
}

//ToDo: Make this work with vecs of more than one function
impl<A: Clone> Applicative<A> for Vec<A> {
    type AHKT<B> = Vec<B>;

    fn pure<B>(a: B) -> Self::AHKT<B> {
        vec![a]
    }

    fn app<B>(&self, other: &Self::AHKT<&dyn Fn(&A) -> B>) -> Self::AHKT<B> {
        (other.fmap(|f| self.fmap(f)))
            .into_iter()
            .flatten()
            .collect()
    }
}

//applicative instance over Option
//applying to None returns None applying None also returns None
impl<A> Applicative<A> for Option<A> {
    type AHKT<B> = Option<B>;

    fn pure<B>(a: B) -> Self::AHKT<B> {
        Some(a)
    }

    fn app<B>(&self, other: &Self::AHKT<&dyn Fn(&A) -> B>) -> Self::AHKT<B> {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(f)) => Some(f(a)),
        }
    }
}
