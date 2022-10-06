pub mod deref;
pub mod deref_mut;
pub mod impl_default;

pub use deref::*;
pub use deref_mut::*;
pub use impl_default::*;
use crate::{AffectHeads, DynamicQueue, Traverse};

impl<T> DynamicQueue<T> {
    pub fn iter<E>(&self) -> Traverse<'_, T, E>
    where
        T: AffectHeads<E>
    {
        Traverse::from(self)
    }
}