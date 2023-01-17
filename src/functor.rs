
//lifting is the cheatcode of type tetris or smth idk
pub trait Functor<A>{
    type FHKT<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B>;
}

impl<A> Functor<A> for Vec<A>{
    type FHKT<B> = Vec<B>;

    fn fmap<B, F: Fn(&A) -> B>(&self, f: F) -> Self::FHKT<B>{
        self.iter().map(&f).collect()
    }
}
