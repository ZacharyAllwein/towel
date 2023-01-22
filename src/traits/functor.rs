/// Trait for function application abstracted over structure
pub trait Functor<A, B> {

    type Mapped;

    fn fmap<F: Fn(A) -> B>(self, f: F) -> Self::Mapped;
}

impl<A, B> Functor<A, B> for Vec<A> {
    type Mapped = Vec<B>;

    fn fmap<F: Fn(A) -> B>(self, f: F) -> Self::Mapped {
        self.into_iter().map(&f).collect()
    }
}

impl<A, B> Functor<A, B> for Option<A> {
    type Mapped = Option<B>;

    fn fmap<F: Fn(A) -> B>(self, f: F) -> Self::Mapped {
        match self {
            Some(x) => Some(f(x)),
            _ => None,
        }
    }
}
