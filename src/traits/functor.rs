
/// Trait for function application abstracted over structure
pub trait Functor<'a, A> {

    /// Generic associated type that allows for higher kinded behavior
    type HKT<B>
    where
        Self: 'a;
    
    /// Applies or lifts function over some structure
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Functor;
    /// 
    /// let v = vec![1, 2, 3];
    /// 
    /// //takes an &i32, adds one too it and turns it into a String
    /// let f: fn(&i32) -> String = |x| (x + 1).to_string();
    ///
    /// assert_eq!(v.fmap(f), vec!["2", "3", "4"]);
    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::HKT<B>;
}

impl<'a, A: 'a> Functor<'a, A> for Vec<A> {
    type HKT<B> = Vec<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::HKT<B> {
        self.iter().map(&f).collect()
    }
}

impl<'a, A: 'a> Functor<'a, A> for Option<A> {
    type HKT<B> = Option<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::HKT<B> {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
