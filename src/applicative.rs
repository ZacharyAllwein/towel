use crate::functor::Functor;

pub trait Applicative<A>: Functor<A> {
    type AHKT<B>;

    type F<B: 'static>: Fn(&A) -> B;

    //signle value into applicative structure
    fn pure(a: A) -> Self;

    fn app<B>(&self, other: &Self::AHKT<Self::F<B>>) -> Self::AHKT<B>;
}

//apllicative instance for Vec behavior : [f, g, h] -> [1, 2, 3, 4] -> [f 1, f 2, f 3, f 4, g 1,
//g 2, g 3 , g 4, h 1, h 2, h 3, h 4]
impl<A: 'static> Applicative<A> for Vec<A> {
    type AHKT<B> = Vec<B>;
    type F<B: 'static> = &'static dyn Fn(&A) -> B;

    fn pure(a: A) -> Self {
        vec![a]
    }

    fn app<B: 'static>(&self, other: &Self::AHKT<Self::F<B>>) -> Self::AHKT<B> {
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
    type F<B: 'static> = fn(&A) -> B;

    fn pure(a: A) -> Self {
        Some(a)
    }
    fn app<B: 'static>(&self, other: &Self::AHKT<Self::F<B>>) -> Self::AHKT<B> {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(f)) => Some(f(a)),
        }
    }
}
