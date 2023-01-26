use crate::prelude::{Applicative, Bound, Functor, Monad};

pub struct Reader<'a, R, A>(Box<dyn FnOnce(R) -> A + 'a>);

impl<'a, R, A, B: 'a> Bound<B> for Reader<'a, R, A> {
    type Bound = Reader<'a, R, B>;
    
    //sets a reader processor that returns 
    //constant value
    fn wrap(a: B) -> Self::Bound {
        Reader(Box::new(move |_| a))
    }
}

impl<'a, R, A, B, F> Functor<A, B, F> for Reader<'a, R, A>
where
    R: 'a,
    A: 'a,
    B: 'a,
    F: 'a + FnOnce(A) -> B,
{
    fn fmap(self, f: F) -> Self::Bound {
        Reader(Box::new(move |r| {

            f(self.eval(r))
        }))
    }
}

impl<'a, R, A, B, C, F> Applicative<A, B, C, F> for Reader<'a, R, A>
where
    R: 'a + Clone,
    A: 'a,
    B: 'a,
    C: 'a,
    F: 'a + FnOnce(A, B) -> C,
{
    type Other = Reader<'a, R, B>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        Reader(Box::new(move |r| {

            f(self.eval(r.clone()), other.eval(r))
        }))
    }
}

impl<'a, R, A, B, F> Monad<A, B, F> for Reader<'a, R, A>
where
    R: 'a + Clone,
    A: 'a,
    B: 'a,
    F: 'a + FnOnce(A) -> Self::Bound,
{
    fn bind(self, f: F) -> Self::Bound {
        Reader(Box::new(move |r| {

            f(self.eval(r.clone())).eval(r)
        }))
    }
}

impl<'a, R, A> Reader<'a, R, A>{
    pub fn eval(self, r: R) -> A{
        (self.0)(r)
    }
}
