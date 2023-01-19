mod data_structures;
mod traits;

pub mod prelude {
    use super::*;
    use traits::*;
    use data_structures::*;
    pub use state::State;
    pub use monad::Monad;
    pub use applicative::Applicative;
    pub use functor::Functor;
    pub use monoid::Monoid;
    pub use semigroup::Semigroup;
}

pub use data_structures::*;
pub use traits::*;
