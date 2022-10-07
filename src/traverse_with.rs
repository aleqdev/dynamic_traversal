use std::ops::Index;

use smallvec::SmallVec;

use crate::{CompiledTraverseInfo, Traverse};

pub trait TraverseWith: Sized {
    fn traverse_with<'a, 'b, E>(
        &'a self,
        info: &'b CompiledTraverseInfo<E>,
    ) -> Traverse<'a, 'b, Self, E>;
}

impl<C> TraverseWith for C
where
    C: Index<usize>,
    <C as Index<usize>>::Output: Sized,
{
    fn traverse_with<'a, 'b, E>(
        &'a self,
        info: &'b CompiledTraverseInfo<E>,
    ) -> Traverse<'a, 'b, Self, E> {
        Traverse {
            collection: self,
            info,
            indices: SmallVec::from_elem(0, info.heads.len()),
            global_index: 0,
        }
    }
}
