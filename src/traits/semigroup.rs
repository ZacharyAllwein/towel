/// A interface for types in which 2 elements of that type can be combined to form
/// one element of that type
///
/// # Vec
///
/// The semigroup for Vectors is concatenation
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
/// The semigroup for options when the Option<T: Semigroup> combines
/// values of types Some. Combining with None is equivalent to an identity function
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
