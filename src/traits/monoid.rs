use crate::traits::Semigroup;

/// Trait for types with [Semigroup] that also have an identity value
///
/// # Vec
/// 
/// Empty for Vec is an empty Vec
///
/// ```
/// # use towel::traits::Monoid;
///
/// assert_eq!(<Vec<i32> as Monoid>::empty(), vec![]);
///
/// ```
///
/// # Option
///
/// Empty for Option is None
///
/// ```
/// # use towel::traits::Monoid;
///
/// assert_eq!(<Option<Vec<i32>> as Monoid>::empty(), None);
pub trait Monoid: Semigroup {

    /// Returns an identity value
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
