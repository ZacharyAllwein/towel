use crate::traits::Bound;

/// Trait for function application embedded in a structure over another structure
pub trait Applicative<A, B, C, F: FnOnce(A, B) -> C>: Bound<C> {
    /// Another version of [Bound] that is applicative specific
    type Other;

    /// Combines values in self and other with same structure with an arbitrary function
    /// then returns something with the same structure and combined values
    ///
    /// ```
    /// # use towel::traits::Applicative;
    /// let v = vec![1, 2];
    /// let u = vec![3, 4];
    ///
    /// //vecs are combined using fn yielding new vec
    /// //that is a product of the original two
    /// assert_eq!(v.lift_a2(u, |a, b| (a, b)), vec![(1, 3), (1, 4), (2, 3), (2,4)]);
    //taking advice from fp complete website
    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound;
}

impl<A: Clone, B: Clone, C, F: Fn(A, B) -> C> Applicative<A, B, C, F> for Vec<A> {
    type Other = Vec<B>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        self.iter()
            .map(|a| other.iter().map(|b| f(a.clone(), b.clone())))
            .flatten()
            .collect()
    }
}

impl<A, B, C, F: FnOnce(A, B) -> C> Applicative<A, B, C, F> for Option<A> {
    type Other = Option<B>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }
}

impl<A, B, C, D, F: FnOnce(A, C) -> D> Applicative<A, C, D, F> for Result<A, B> {
    type Other = Result<C, B>;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        match (self, other) {
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            (Ok(a), Ok(b)) => Ok(f(a, b)),
        }
    }
}
