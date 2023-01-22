///
/// ```
/// # use towel::traits::Semigroup;
///
/// let v = vec![1, 2, 3];
/// let u = vec![4, 5, 6];
///
/// assert_eq!(v.combine(u), vec![1, 2, 3, 4, 5, 6]);
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
/// assert_eq!(o.clone().combine(p.clone()), Some(vec![1, 2, 3, 4, 5, 6]));
///
/// //returns Some value
/// assert_eq!(o.clone().combine(None), o);
///
/// //returns Some value
/// assert_eq!((None).combine(p.clone()), p);
pub trait Semigroup: Sized {
    /// Combines two elements of a type that impls Semigroup
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
