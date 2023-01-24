use crate::traits::Bound;

/// Trait for function application abstracted over structure
///
/// # Vector
///
/// fmap for vec is the same as calling vec.iter().map().collect()
///
/// ```
/// # use towel::traits::Functor;
///
/// let v = vec![1, 2, 3];
///
/// assert_eq!(v.fmap(|x| (x + 1).to_string()), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
/// ```
///
/// # Option
///
/// fmap for option abstracts over matching Some and None and applying fn to inner value of Some
///
/// ```
/// # use towel::traits::Functor;
///
/// let a = Some(3);
/// let f: fn(i32) -> String = |a| (a + 1).to_string();
///
/// assert_eq!(a.fmap(f), Some("4".to_string()));
/// assert_eq!(None.fmap(f), None);
pub trait Functor<A, B, F: Fn(A) -> B>: Bound<B> {
    ///Lifts a function over a structure.
    ///Changes values inside structure, but not structure itself.
    fn fmap(self, f: F) -> Self::Bound;
}

impl<A, B, F: Fn(A) -> B> Functor<A, B, F> for Vec<A> {
    fn fmap(self, f: F) -> Self::Bound {
        self.into_iter().map(&f).collect()
    }
}

impl<A, B, F: Fn(A) -> B> Functor<A, B, F> for Option<A> {
    fn fmap(self, f: F) -> Self::Bound {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
