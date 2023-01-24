use crate::traits::Bound;

/// Trait for function application embedded in a structure over another structure
///
/// # Vector
///
/// pure for vec puts one value into a vector
/// ```
/// use towel::traits::Applicative;
///
/// assert_eq!(<Vec<i32> as Applicative<i32, i32, i32, fn(i32, i32) -> i32>>::pure(1), vec![1]);
/// ```
///
/// lift_a2 over two vecs combines each element in the first vec with each element in the second
/// in a one dimensional vec using the fn provided to lift_a2
///
/// ```
/// # use towel::traits::Applicative;
///
/// let v = vec![1, 2];
/// let u = vec![3, 4];
///
/// assert_eq!(v.lift_a2(u, |a, b| (a, b)), vec![(1, 3), (1, 4), (2, 3), (2,4)]);
/// ```
///
/// # Option
///
/// pure for option lifts a value into Some
///
/// ```
/// # use towel::traits::Applicative;
///
/// assert_eq!(<Option<i32> as Applicative<i32, i32, i32, fn(i32, i32) -> i32>>::pure(2), Some(2));
/// ```
///
/// lift_a2 over Option combines two Some values into one joining their internal values with
/// the function for lift_a2 or returns None if either self or other is None
///
/// ```
/// # use towel::traits::Applicative;
///
/// let f: fn(i32, i32) -> (i32, i32) = |a, b| (a, b);
///
/// assert_eq!(Some(1).lift_a2(Some(2), f), Some((1, 2)));
/// assert_eq!(None.lift_a2(Some(1), f), None);
pub trait Applicative<A, B, C, F: Fn(A, B) -> C>: Bound<C> {
    /// Another version of [Bound] that is applicative specific
    type Other;

    /// Lifts value into an applicative context
    fn pure(a: C) -> Self::Bound;

    /// Combines values in self and other with same structure with an arbitrary function
    /// then reuttrns something with the same structure and combined values
    //taking advice from fp complete website
    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound;
}

impl<A: Clone, B: Clone, C, F: Fn(A, B) -> C> Applicative<A, B, C, F> for Vec<A> {
    type Other = Vec<B>;

    fn pure(a: C) -> Self::Bound {
        vec![a]
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        self.iter()
            .map(|a| other.iter().map(|b| f(a.clone(), b.clone())))
            .flatten()
            .collect()
    }
}

impl<A, B, C, F: Fn(A, B) -> C> Applicative<A, B, C, F> for Option<A> {
    type Other = Option<B>;

    fn pure(a: C) -> Self::Bound {
        Some(a)
    }

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }
}
