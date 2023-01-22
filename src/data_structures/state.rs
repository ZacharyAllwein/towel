use crate::prelude::*;

pub struct State<S, A>(Box<dyn FnOnce(S) -> (A, S)>);

impl<S, A> Functor<A> for State<S, A> {
    type HKT<B> = State<S, B>;

    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::HKT<B> {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(a), ns)
        }))
    }
}

impl<S, A> Applicative<A> for State<S, A> {

    fn pure<B>(a: B) -> Self::HKT<B>{
        State(Box::new(move |s| (a, s)))
    }

    fn lift_a2<B, C, F: Fn(A, B) -> C>(self, other: Self::HKT<B>, f: F) -> Self::HKT<C>{
        State(Box::new(move |s| {

            let (a, ns) = self.eval(s);
            let (b, fs) = other.eval(ns);

            (f(a, b), fs)
        }))
    }
}

impl<S, A> Monad<A> for State<S, A> {
    fn bind<B, F: Fn(A) -> Self::HKT<B>>(self, f: F) -> Self::HKT<B> {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            f(a).eval(ns)
        }))
    }
}

impl<S, A> State<S, A> {
    pub fn new(s: Box<dyn FnOnce(S) -> (A, S)>) -> Self {
        State(s)
    }
    pub fn eval(self, s: S) -> (A, S) {
        (self.0)(s)
    }
}
