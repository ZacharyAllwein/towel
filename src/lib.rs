mod data_structures;
mod traits;

pub mod prelude {
    use super::*;
    pub use applicative::Applicative;
    use data_structures::*;
    pub use functor::Functor;
    pub use monad::Monad;
    pub use monoid::Monoid;
    pub use semigroup::Semigroup;
    pub use state::State;
    use traits::*;
}

pub use data_structures::*;
pub use traits::*;
