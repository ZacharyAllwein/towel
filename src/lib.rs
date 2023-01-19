mod data_structures;
mod traits;


pub use data_structures::*;
pub use traits::*;

pub mod prelude {
    use super::*;
    pub use applicative::Applicative;
    pub use functor::Functor;
    pub use monad::Monad;
    pub use monoid::Monoid;
    pub use semigroup::Semigroup;
    pub use state::State;
    pub use cons::Cons;
}
