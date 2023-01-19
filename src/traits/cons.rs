
//a immutable interface for dealing with list like data structures
//probably not a good idea, but I'm just having fun
pub trait Cons<A>{
    
    fn layer(self, a: A) -> Self;

    fn next(self) -> (Option<A>, Self);

}

impl<A> Cons<A> for Vec<A>{

    fn layer(mut self, a: A) -> Self{
        self.push(a);

        self
    }

    fn next(mut self) -> (Option<A>, Self){
        (self.pop(), self)
    }

}
