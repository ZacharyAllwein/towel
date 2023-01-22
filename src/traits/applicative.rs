use crate::traits::Functor;

/// Trait for function application embedded in a structure over another structure
pub trait Applicative<'a, A>: Functor<'a, A> {

    /// Puts a single value into structure of applicative
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Applicative;
    ///
    /// let p = <Vec<i32>>::pure(1);
    ///
    /// assert_eq!(p, vec![1]);
    fn pure(a: A) -> Self;
    
    //taking advice from fp complete website
    fn lift_a2<B: 'a, C: 'a, F: Fn(&A, &B) -> C + 'a>(&'a self, other: &'a Self::HKT<B>, f: F) -> Self::HKT<C>;
}

impl<'a, A: 'a> Applicative<'a, A> for Vec<A> {

    fn pure(a: A) -> Self {
        vec![a]
    }

    fn lift_a2<B: 'a, C: 'a, F: Fn(&A, &B) -> C + 'a>(&self, other: &Self::HKT<B>, f: F) -> Self::HKT<C> {
        self.iter()
            .map(|a| other.iter()
                         .map(|b| f(a, b)))
            .flatten()
            .collect()
    }
}

impl<'a, A: 'a> Applicative<'a, A> for Option<A> {

    fn pure(a: A) -> Self {
        Some(a)
    }

    fn lift_a2<B: 'a, C: 'a, F: Fn(&A, &B) -> C + 'a>(&self, other: &Self::HKT<B>, f: F) -> Self::HKT<C> {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }
}
