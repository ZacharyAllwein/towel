//lifting is the cheatcode of type tetris or smth idk
pub trait Functor<'a, A> {
    type FHKT<B>
    where
        Self: 'a;

    fn fmap<B, F: Fn(&A) -> B + 'a>(&'a self, f: F) -> Self::FHKT<B>;
}

//functor instance for generic vector, wrapper over iter().collect()
impl<'a, A: 'a> Functor<'a, A> for Vec<A> {
    type FHKT<B> = Vec<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B> {
        self.iter().map(&f).collect()
    }
}

//functor for option lifts over context of checking for nonexistance
impl<'a, A: 'a> Functor<'a, A> for Option<A> {
    type FHKT<B> = Option<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B> {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
