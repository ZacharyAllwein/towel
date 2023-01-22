use crate::prelude::*;

pub struct State<'a, S, A>(Box<dyn Fn(S) -> (A, S) + 'a>);

impl<'a, S: 'a, A: 'a> Functor<'a, A> for State<'a, S, A> {
    type HKT<B: 'a> = State<'a, S, B>;

    fn fmap<B: 'a, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::HKT<B> {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(&a), ns)
        }))
    }
}

impl<'a, S: 'a, A: Clone + 'a> Applicative<'a, A> for State<'a, S, A> {

    fn pure(a: A) -> State<'a, S, A> {
        State(Box::new(move |s| (a.clone(), s)))
    }

    fn lift_a2<B: 'a, C: 'a, F: Fn(&A, &B) -> C + 'a>(&'a self, other: &'a Self::HKT<B>, f: F) -> Self::HKT<C>{
        State(Box::new(move |s| {

            let (a, ns) = self.eval(s);
            let (b, fs) = other.eval(ns);

            (f(&a, &b), fs)
        }))
    }
}

impl<'a, S: 'a, A: Clone + 'a> Monad<'a, A> for State<'a, S, A> {
    fn bind<B: 'a, F: Fn(&A) -> Self::HKT<B> + 'a>(&'a self, f: F) -> Self::HKT<B> {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            f(&a).eval(ns)
        }))
    }
}

impl<'a, S, A> State<'a, S, A> {
    pub fn new(s: Box<dyn Fn(S) -> (A, S) + 'a>) -> Self {
        State(s)
    }
    pub fn eval(&self, s: S) -> (A, S) {
        (self.0)(s)
    }
}
