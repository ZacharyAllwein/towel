/// Trait for types with an associative operation that allows two elements to be combined
///
/// # Vec
///
/// Semigroup behavior for vec is append
///
/// ```
/// # use towel::traits::Semigroup;
///
/// let v = vec![1, 2, 3];
/// let u = vec![4, 5, 6];
///
/// assert_eq!(v.combine(&u), vec![1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Option
///
/// The semigroup for options allows to options to be combined
/// when Option<A: Semigroup>. In the case of a None variant it
/// returns the other operand
///
/// ```
/// # use towel::traits::Semigroup;
///
/// let o = Some(vec![1, 2, 3]);
/// let p = Some(vec![4, 5, 6]);
///
/// //relies on Vec's Semigroup impl
/// assert_eq!(o.combine(&p), Some(vec![1, 2, 3, 4, 5, 6]));
///
/// //returns Some value
/// assert_eq!(o.combine(&None), o);
///
/// //returns Some value
/// assert_eq!((None).combine(&p), p);
pub trait Semigroup: Sized {
    /// Combines two elements of a type that impls Semigroup
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
