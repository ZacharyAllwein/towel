use crate::traits::Functor;

/// Trait for function application embedded in a structure over another structure
pub trait Applicative<'a, A>: Functor<'a, A> {
    
    /// Embeded function type, allows implementor to choose between
    /// static dispatch if only one function is embedded in structure
    /// or dynamic if there will be multiple
    type F<B: 'a>: Fn(&A) -> B
    where
        Self: 'a;

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
    
    /// Applys strcutrue with embedded function to structure with value
    /// 
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Applicative;
    ///
    /// let v = vec![1, 2, 3];
    /// let vf: Vec<&dyn Fn(&i32) -> i32> = vec![&|x| x + 1, &|x| x * 2];
    ///
    /// assert_eq!(v.apply(&vf), vec![2, 3, 4, 2, 4, 6]);
    fn apply<B: 'a>(&'a self, other: &'a Self::HKT<Self::F<B>>) -> Self::HKT<B>;
}

impl<'a, A: 'a> Applicative<'a, A> for Vec<A> {
    type F<B: 'a> = &'a dyn Fn(&A) -> B;

    fn pure(a: A) -> Self {
        vec![a]
    }

    fn apply<B: 'a>(&self, other: &Self::HKT<Self::F<B>>) -> Self::HKT<B> {
        (other.fmap(|f| self.fmap(f)))
            .into_iter()
            .flatten()
            .collect()
    }
}

impl<'a, A: 'a> Applicative<'a, A> for Option<A> {
    type F<B: 'a> = fn(&A) -> B;

    fn pure(a: A) -> Self {
        Some(a)
    }
    fn apply<B: 'a>(&self, other: &Self::HKT<Self::F<B>>) -> Self::HKT<B> {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(f)) => Some(f(a)),
        }
    }
}
