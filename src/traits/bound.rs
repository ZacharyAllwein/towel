pub trait Bound<B>{
    type Bound;
}

impl<A, B> Bound<B> for Vec<A>{
    type Bound = Vec<B>;
}

impl<A, B> Bound<B> for Option<A>{
    type Bound = Option<B>;
}
