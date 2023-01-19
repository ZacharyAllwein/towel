use crate::prelude::*;

pub struct State<'a, S, A>(Box<dyn Fn(S) -> (A, S) + 'a>);

impl<'a, S: 'a, A: 'a> Functor<'a, A> for State<'a, S, A> {
    type FHKT<B> = State<'a, S, B>;

    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::FHKT<B> {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(&a), ns)
        }))
    }
}

impl<'a, S: Monoid + 'a, A: Clone + 'a> Applicative<'a, A> for State<'a, S, A> {
    type AHKT<B: 'a> = State<'a, S, B>;
    type F<B: 'a> = fn(&A) -> B;

    fn pure(a: A) -> State<'a, S, A> {
        State(Box::new(move |_| (a.clone(), <S as Monoid>::mempty())))
    }

    fn app<B: 'a>(&'a self, other: &'a Self::AHKT<Self::F<B>>) -> Self::AHKT<B> {
        State(Box::new(move |s| {
            let (fa, ns) = other.eval(s);
            let (a, fs) = self.eval(ns);

            (fa(&a), fs)
        }))
    }
}

impl<'a, S, A> State<'a, S, A> {
    pub fn new(s: Box<dyn Fn(S) -> (A, S) + 'a>) -> State<'a, S, A> {
        State(s)
    }
    pub fn eval(&self, s: S) -> (A, S) {
        (self.0)(s)
    }
}
