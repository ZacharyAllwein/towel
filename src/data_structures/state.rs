use crate::prelude::{Applicative, Bound, Functor, Monad};

pub struct State<'a, S, A>(Box<dyn FnOnce(S) -> (A, S) + 'a>);

impl<'a, S: 'a, A: 'a, B: 'a> Bound<B> for State<'a, S, A> {
    type Bound = State<'a, S, B>;
}

impl<'a, S: 'a, A: 'a, B: 'a, F: 'a + Fn(A) -> B> Functor<A, B, F> for State<'a, S, A> {
    fn fmap(self, f: F) -> Self::Bound {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(a), ns)
        }))
    }
}

impl<'a, S: 'a, A: 'a, B: 'a, C: 'a, F> Applicative<A, B, C, F> for State<'a, S, A>
where
    F: 'a + Fn(A, B) -> C,
{
    type Other = State<'a, S, B>;

    fn pure(a: C) -> Self::Bound {
        State(Box::new(move |s| (a, s)))
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            let (b, fs) = other.eval(ns);

            (f(a, b), fs)
        }))
    }
}

impl<'a, S: 'a, A: 'a, B: 'a, F> Monad<A, B, F> for State<'a, S, A>
where
    F: 'a + Fn(A) -> Self::Bound,
{
    fn ret(a: B) -> Self::Bound {
        <Self as Applicative<A, A, B, fn(A, A) -> B>>::pure(a)
    }
    fn bind(self, f: F) -> Self::Bound {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            f(a).eval(ns)
        }))
    }
}

impl<'a, S, A> State<'a, S, A> {
    pub fn new(s: Box<dyn FnOnce(S) -> (A, S)>) -> Self {
        State(s)
    }
    pub fn eval(self, s: S) -> (A, S) {
        (self.0)(s)
    }
}
