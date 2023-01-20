/// Immutable interface for handling list like data structures
/// note: direction of list only maintained through usage of Cons functions
pub trait Cons<A> {

    /// Produces new data structure from old Cons and a new value
    /// Takes ownership of old Cons
    ///
    /// # Example
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Cons;
    /// 
    /// let v = vec![1, 2, 3];
    ///
    /// //u takes ownership of v
    /// let u = v.layer(4);
    /// 
    /// //layer adds to end of vec
    /// assert_eq!(u, vec![1, 2, 3, 4]);
    fn layer(self, a: A) -> Self;
    
    /// Produces tuple containing first value in Cons and tail of Cons
    ///
    /// # Example
    ///
    /// ```
    /// use towel::traits::Cons;
    ///
    /// let (x, v) = vec![1, 2, 3].next();
    /// 
    /// //head of Cons is last of vec
    /// assert_eq!(x, Some(3));
    ///
    /// //tail of Cons is vec without last value
    /// assert_eq!(v, vec![1, 2]);
    fn next(self) -> (Option<A>, Self);
}

impl<A> Cons<A> for Vec<A> {
    fn layer(mut self, a: A) -> Self {
        self.push(a);

        self
    }

    fn next(mut self) -> (Option<A>, Self) {
        (self.pop(), self)
    }
}
