//lifting is the cheatcode of type tetris or smth idk
pub trait Functor<A> {
    type FHKT<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B>;
}

//functor instance for generic vector, wrapper over iter().collect()
impl<A> Functor<A> for Vec<A> {
    type FHKT<B> = Vec<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B> {
        self.iter().map(&f).collect()
    }
}

//functor for option lifts over context of checking for nonexistance
impl<A> Functor<A> for Option<A> {
    type FHKT<B> = Option<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B> {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
