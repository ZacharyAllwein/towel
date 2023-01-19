use crate::traits::functor::Functor;

pub trait Applicative<'a, A>: Functor<'a, A> {

    type F<B: 'a>: Fn(&A) -> B

    where
        Self: 'a;

    //signle value into applicative structure
    fn pure(a: A) -> Self;

    fn app<B: 'a>(&'a self, other: &'a Self::HKT<Self::F<B>>) -> Self::HKT<B>;
}

//apllicative instance for Vec behavior : [f, g, h] -> [1, 2, 3, 4] -> [f 1, f 2, f 3, f 4, g 1,
//g 2, g 3 , g 4, h 1, h 2, h 3, h 4]
impl<'a, A: 'a> Applicative<'a, A> for Vec<A> {
    type F<B: 'a> = &'a dyn Fn(&A) -> B;

    fn pure(a: A) -> Self {
        vec![a]
    }

    fn app<B: 'a>(&self, other: &Self::HKT<Self::F<B>>) -> Self::HKT<B> {
        (other.fmap(|f| self.fmap(f)))
            .into_iter()
            .flatten()
            .collect()
    }
}

//applicative instance over Option
//applying to None returns None applying None also returns None
impl<'a, A: 'a> Applicative<'a, A> for Option<A> {
    type F<B: 'a> = fn(&A) -> B;

    fn pure(a: A) -> Self {
        Some(a)
    }
    fn app<B: 'a>(&self, other: &Self::HKT<Self::F<B>>) -> Self::HKT<B> {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(f)) => Some(f(a)),
        }
    }
}
