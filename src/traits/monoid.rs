use crate::traits::Semigroup;

/// Trait for types with [Semigroup] that also have an identity value
///
///
pub trait Monoid: Semigroup {
    /// Returns an identity value
    ///
    /// ```
    /// # use towel::traits::Monoid;
    /// //identity value for Option is None
    /// assert_eq!(<Option<Vec<i32>> as Monoid>::empty(), None);
    fn empty() -> Self;
}

impl<A: Clone> Monoid for Vec<A> {
    fn empty() -> Self {
        vec![]
    }
}

impl<A: Semigroup + Clone> Monoid for Option<A> {
    fn empty() -> Self {
        None
    }
}
