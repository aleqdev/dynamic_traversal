use smallvec::smallvec;

use crate::{AffectHeads, DynamicQueue, Head, Traverse};

impl<'a, T, E> From<&'a DynamicQueue<T>> for Traverse<'a, T, E>
where
    T: AffectHeads<E>,
{
    fn from(queue: &'a DynamicQueue<T>) -> Self {
        Self {
            queue,
            heads: smallvec![Head::default()],
        }
    }
}
