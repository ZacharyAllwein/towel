use crate::prelude::{Applicative, Functor, Monad};

pub struct State<'a, S, A>(Box<dyn FnOnce(S) -> (A, S) + 'a>);

impl<'a, S: 'a, A: 'a, B: 'a, F: 'a + Fn(A) -> B> Functor<A, B, F> for State<'a, S, A> {
    type Mapped = State<'a, S, B>;

    fn fmap(self, f: F) -> Self::Mapped {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(a), ns)
        }))
    }
}

impl<'a, S: 'a, A: 'a, B: 'a, C: 'a, F, G> Applicative<A, B, C, F, G> for State<'a, S, A>
where F: 'a + Fn(A, B) -> C,
      G: 'a + Fn(A) -> C + 'a{
    type Other = State<'a, S, B>;

    fn pure(a: C) -> Self::Mapped {
        State(Box::new(move |s| (a, s)))
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Mapped {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            let (b, fs) = other.eval(ns);

            (f(a, b), fs)
        }))
    }
}

impl<'a, S: 'a, A: 'a, B: 'a, F, G, H> Monad<A, B, F, G, H> for State<'a, S, A> 
where F: 'a + Fn(A) -> Self::Mapped,
      G: 'a + Fn(A, A) -> B,
      H: 'a + Fn(A) -> B{
    fn bind(self, f: F) -> Self::Mapped {
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
