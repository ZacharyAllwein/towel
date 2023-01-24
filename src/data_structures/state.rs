use crate::prelude::{Applicative, Bound, Functor, Monad};

/// State struct that implements Functor, Applicative and Monad
/// allowing for functional state management
///
/// # Example
///
/// ```
/// # use towel::data_structures::State;
/// # use towel::traits::{Monad, Functor};
///
/// //|_| ((), 3)
/// let initial = State::<i32, i32>::put(3i32);
///
/// //|_| (3, 3)
/// let second = initial.bind(|_| State::<i32, i32>::get());
///
/// //|_| ("4", 3)
/// let third = second.fmap(|a| (a + 1).to_string());
///
/// assert_eq!(third.eval(0i32), ("4".to_string(), 3));
pub struct State<'a, S, A>(Box<dyn FnOnce(S) -> (A, S) + 'a>);

impl<'a, S: 'a, A: 'a, B: 'a> Bound<B> for State<'a, S, A> {
    type Bound = State<'a, S, B>;

    fn wrap(a: B) -> Self::Bound {
        State(Box::new(move |s| (a, s)))
    }
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
    fn bind(self, f: F) -> Self::Bound {
        State(Box::new(move |s| {
            let (a, ns) = self.eval(s);
            f(a).eval(ns)
        }))
    }
}

impl<'a, S: 'a, A: 'a> State<'a, S, A> {
    /// Sets an initial state
    pub fn put(s: S) -> State<'a, S, ()> {
        State(Box::new(move |_| ((), s)))
    }

    /// Runs the state processor on some initial state
    pub fn eval(self, s: S) -> (A, S) {
        (self.0)(s)
    }
}

//Got to have some way to copy that state
impl<'a, S: 'a + Clone, A: 'a> State<'a, S, A> {
    /// Sets result value to state
    pub fn get() -> State<'a, S, S> {
        State(Box::new(|s| (s.clone(), s)))
    }
}
