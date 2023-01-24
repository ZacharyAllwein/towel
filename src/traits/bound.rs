/// Trait that encapsulates higher kinded behavior. Helps to unify the
/// return values of function from Functor, Applicative, and Monad trait
/// and acts as pure or return
pub trait Bound<B> {
    /// Typically generic output type
    ///
    /// ```
    /// # use towel::traits::Bound;
    /// //Bound lets us encapsulate the idea that the Ouput: Vec<String> is related to the input:
    /// //Vec<i32>
    /// let a = <Vec<i32> as Bound<String>>::wrap("foo".to_string());
    ///
    /// //[Some("foo")]
    /// println!("{:?}", a);
    type Bound;

    /// Wraps a value into some structure that is the type Bound
    ///
    /// ```
    /// # use towel::traits::Bound;
    /// let a = <Option<i32> as Bound<&'static str>>::wrap("hello");
    ///
    /// assert_eq!(a, Some("hello"));
    fn wrap(a: B) -> Self::Bound;
}

impl<A, B> Bound<B> for Vec<A> {
    type Bound = Vec<B>;

    fn wrap(a: B) -> Self::Bound {
        vec![a]
    }
}

impl<A, B> Bound<B> for Option<A> {
    type Bound = Option<B>;

    fn wrap(a: B) -> Self::Bound {
        Some(a)
    }
}
