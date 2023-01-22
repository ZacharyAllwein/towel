use crate::traits::Functor;

/// Trait for function application embedded in a structure over another structure
pub trait Applicative<A, B, C>: Functor<A, C> {

    type Other;

    fn pure(a: C) -> Self::Mapped;

    //taking advice from fp complete website
    fn lift_a2<F: Fn(A, B) -> C>(self, other: Self::Other, f: F) -> Self::Mapped;
}

impl<A: Clone, B: Clone, C> Applicative<A, B, C> for Vec<A>{

    type Other = Vec<B>;

    fn pure(a: C) -> Self::Mapped {
        vec![a]
    }

    fn lift_a2<F: Fn(A, B) -> C>(self, other: Self::Other, f: F) -> Self::Mapped {
        self.iter()
            .map(|a| other.iter()
                         .map(|b| f(a.clone(), b.clone())))
            .flatten()
            .collect()
    }
}

impl<A, B, C> Applicative<A, B, C> for Option<A> {

    type Other = Option<B>;

    fn pure(a: C) -> Self::Mapped {
        Some(a)
    }

    fn lift_a2<F: Fn(A, B) -> C>(self, other: Self::Other, f: F) -> Self::Mapped {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }
}
