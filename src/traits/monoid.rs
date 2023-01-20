use crate::traits::Semigroup;

/// Trait for types with [Semigroup] that also have an identity value
pub trait Monoid: Semigroup {
    
    /// Returns an identity value from a Type with a Monoid
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Monoid;
    /// 
    /// //empty for vecs
    /// assert_eq!(<Vec<i32> as Monoid>::empty(), vec![]);
    ///
    /// //empty for options
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
