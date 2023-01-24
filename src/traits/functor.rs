use crate::traits::Bound;

/// Trait for function application abstracted over structure
pub trait Functor<A, B, F: FnOnce(A) -> B>: Bound<B> {
    ///Lifts a function over a structure.
    ///Changes values inside structure, but not structure itself.
    ///
    /// ```
    /// # use towel::traits::Functor;
    /// let v = vec![1, 2, 3];
    ///
    /// //lifts fn over vec changing values but leaving structure
    /// //the same
    /// assert_eq!(v.fmap(|x| (x + 1).to_string()), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
    /// ```
    fn fmap(self, f: F) -> Self::Bound;
}

impl<A, B, F: Fn(A) -> B> Functor<A, B, F> for Vec<A> {
    fn fmap(self, f: F) -> Self::Bound {
        self.into_iter().map(&f).collect()
    }
}

impl<A, B, F: FnOnce(A) -> B> Functor<A, B, F> for Option<A> {
    fn fmap(self, f: F) -> Self::Bound {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
