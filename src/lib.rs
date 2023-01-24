pub mod combinators;
pub mod data_structures;
pub mod traits;

pub mod prelude {
    use super::*;
    pub use combinators::*;
    pub use data_structures::Either::{self, Left, Right};
    pub use data_structures::State;
    pub use traits::Applicative;
    pub use traits::Bound;
    pub use traits::Cons;
    pub use traits::Functor;
    pub use traits::Monad;
    pub use traits::Monoid;
    pub use traits::Semigroup;
}
