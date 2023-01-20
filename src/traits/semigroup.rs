
/// Trait for types with an associative operation that allows two elements to be combined
pub trait Semigroup: Sized {

    /// Combines two elements of a type that impls Semigroup
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use towel::traits::Semigroup;
    ///
    /// let v = vec![1, 2, 3];
    /// let u = vec![4, 5, 6];
    ///
    /// assert_eq!(v.combine(&u), vec![1, 2, 3, 4, 5, 6]);
    fn combine(&self, other: &Self) -> Self;
}

impl<A: Clone> Semigroup for Vec<A> {
    fn combine(&self, other: &Self) -> Self {
        let mut new = self.clone();
        new.extend_from_slice(other);
        new
    }
}

impl<A: Semigroup + Clone> Semigroup for Option<A> {
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (None, x) => x.clone(),
            (x, None) => x.clone(),
            (Some(x), Some(y)) => Some(x.combine(&y)),
        }
    }
}
