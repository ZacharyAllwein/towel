pub trait Semigroup: Sized {
    fn combine(&self, other: &Self) -> Self;
}

impl<A: Clone> Semigroup for Vec<A> {
    fn combine(&self, other: &Self) -> Self {
        let mut new = self.clone();
        new.extend_from_slice(other);
        new
    }
}
