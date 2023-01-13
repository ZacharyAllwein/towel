pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

trait Functor<A> {
    type F<B>;
    fn fmap<C, F: FnMut(A) -> C>(self, f: F) -> Self::F<C>;
}

trait Applicative<A> {
    type F<B>;
    fn app<C, F : FnMut(A) -> C>(self, f: Self::F<F>) -> Self::F<C>;
}

#[derive(Debug)]
struct Pair<A, B> (A, B);

impl<A, B> Functor<B> for Pair<A, B> {
    type F<C> = Pair<A, C>;
    
    fn fmap<D, F: FnMut(B) -> D>(self, mut f: F) -> Self::F<D>{
        match self {
            Pair (a, b) => Pair (a, f(b))
        }
    }
}

impl<A, B> Applicative<B> for Pair<A, B> {
    type F<C> = Pair<A, C>;
    
    fn app<D, F: FnMut(B) -> D>(self, f: Self::F<F>) -> Self::F<D>{
        match f{
            Pair(a, mut g) => Pair(a, g(self.1))
        }
    }
}

impl<A> Functor<A> for Option<A> {
    type F<B> = Option<B>;
    fn fmap<C, F: FnMut(A) -> C>(self, f: F) -> Self::F<C> {
        self.map(f)
    }
}

impl<A> Functor<A> for Vec<A> {
    type F<B> = Vec<B>;
    fn fmap<C, F: FnMut(A) -> C>(self, f: F) -> Self::F<C> {
        self.into_iter().map(f).collect()
    }
}

fn main(){
    let x = Pair(1, 2).app(Pair(1, |x: i32| x.to_string()));
    println!("{:?}", x)
}
