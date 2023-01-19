pub mod data_structures;
pub mod traits;

pub mod prelude {
    use super::*;
    pub use traits::Applicative;
    pub use traits::Functor;
    pub use traits::Monad;
    pub use traits::Monoid;
    pub use traits::Semigroup;
    pub use data_structures::State;
    pub use traits::Cons;
}
