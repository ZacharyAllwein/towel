//lifting is the cheatcode of type tetris or smth idk
pub trait Functor<'a, A> {
    type HKT<B>
    where
        Self: 'a;

    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::HKT<B>;
}

//functor instance for generic vector, wrapper over iter().collect()
impl<'a, A: 'a> Functor<'a, A> for Vec<A> {
    type HKT<B> = Vec<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::HKT<B> {
        self.iter().map(&f).collect()
    }
}

//functor for option lifts over context of checking for nonexistance
impl<'a, A: 'a> Functor<'a, A> for Option<A> {
    type HKT<B> = Option<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::HKT<B> {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
