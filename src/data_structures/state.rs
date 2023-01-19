use crate::functor::Functor;

struct State<'a, S, A>(Box<dyn Fn(S) -> (A, S) + 'a>);


impl<'a, S: 'a, A: 'a> Functor<'a, A> for State<'a, S, A> {

    type FHKT<B> = State<'a, S, B>;

    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::FHKT<B>
    {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            (f(&a), ns)
        }))
    }
}

impl<'a, S, A> State<'a, S, A> {
    pub fn eval(&self, s: S) -> (A, S) {
        (self.0)(s)
    }
}
