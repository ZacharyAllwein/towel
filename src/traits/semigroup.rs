/// A interface for types in which 2 elements of that type can be combined to form
/// one element of that type
pub trait Semigroup: Sized {

    /// Combines two elements of one type into one element of that type
    ///
    /// ```
    /// # use towel::traits::Semigroup;
    /// let o = Some(vec![1, 2, 3]);
    /// let p = Some(vec![4, 5, 6]);
    ///
    /// //Option Semigroup relies on Vec's Semigroup
    /// assert_eq!(o.clone().combine(p.clone()), Some(vec![1, 2, 3, 4, 5, 6]));
    ///
    /// //combining None and Some works as identity for Some value
    /// assert_eq!(o.clone().combine(None), o);
    ///
    /// assert_eq!((None).combine(p.clone()), p);
    fn combine(self, other: Self) -> Self;
}

impl<A: Clone> Semigroup for Vec<A> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl<A: Semigroup + Clone> Semigroup for Option<A> {
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (None, x) => x,
            (x, None) => x,
            (Some(x), Some(y)) => Some(x.combine(y)),
        }
    }
}
