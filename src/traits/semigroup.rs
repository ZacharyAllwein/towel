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

//semigroup for option relies on inner semigroup for A
impl<A: Semigroup + Clone> Semigroup for Option<A> {
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (None, x) => x.clone(),
            (x, None) => x.clone(),
            (Some(x), Some(y)) => Some(x.combine(&y)),
        }
    }
}
