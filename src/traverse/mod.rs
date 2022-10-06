pub mod impls;
pub use impls::*;

use smallvec::SmallVec;

use crate::{AffectHeads, DynamicQueue};

#[derive(Debug, Clone)]
pub struct Head<T> {
    pub index: usize,
    pub effects: SmallVec<[T; 4]>,
}

impl<T> Default for Head<T> {
    fn default() -> Self {
        Self {
            index: Default::default(),
            effects: Default::default(),
        }
    }
}

pub type Heads<T> = SmallVec<[Head<T>; 4]>;

pub type TraverseBuffer<'a, 'b, T, E> = &'a SmallVec<[(&'b T, &'b Head<E>); 4]>;

#[derive(Debug)]
pub struct Traverse<'a, T, E>
where
    T: AffectHeads<E>,
{
    queue: &'a DynamicQueue<T>,
    heads: Heads<E>,
}
