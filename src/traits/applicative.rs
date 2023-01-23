use crate::traits::Bound;

/// Trait for function application embedded in a structure over another structure
pub trait Applicative<A, B, C, F: Fn(A, B) -> C>: Bound<C>{

    type Other;

    fn pure(a: C) -> Self::Bound;

    //taking advice from fp complete website
    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound;
}

impl<A: Clone, B: Clone, C, F: Fn(A, B) -> C> Applicative<A, B, C, F> for Vec<A>{

    type Other = Vec<B>;

    fn pure(a: C) -> Self::Bound {
        vec![a]
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        self.iter()
            .map(|a| other.iter().map(|b| f(a.clone(), b.clone())))
            .flatten()
            .collect()
    }
}

impl<A, B, C, F: Fn(A, B) -> C> Applicative<A, B, C, F> for Option<A> {

    type Other = Option<B>;

    fn pure(a: C) -> Self::Bound {
        Some(a)
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }
}
