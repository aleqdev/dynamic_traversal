use std::ops::Index;

use smallvec::SmallVec;

use crate::CompiledTraverseInfo;

pub struct Traverse<'a, 'b, C, E> {
    pub collection: &'a C,
    pub info: &'b CompiledTraverseInfo<E>,
    pub indices: SmallVec<[usize; 4]>,
    pub global_index: usize,
}

impl<'a, 'b, C, E> Iterator for Traverse<'a, 'b, C, E>
where
    C: Index<usize>,
    <C as Index<usize>>::Output: Sized,
    E: 'b,
{
    type Item = Box<[Option<(&'a <C as Index<usize>>::Output, &'b SmallVec<[E; 4]>)>]>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c = Vec::with_capacity(self.info.heads.len());

        let mut has_some = false;

        for (head, index) in self.info.heads.iter().zip(self.indices.iter_mut()) {
            c.push(
                (head.offset <= self.global_index && *index < head.indices.len()).then(|| {
                    has_some = true;
                    self.global_index += 1;
                    *index += 1;
                    (&self.collection[head.indices[*index - 1]], &head.effects)
                }),
            );
        }

        has_some.then_some(c.into_boxed_slice())
    }
}
