use smallvec::{smallvec, SmallVec};

use crate::CompiledHeadInfo;

#[derive(Debug, Clone)]
pub struct CompiledTraverseInfo<E> {
    pub heads: SmallVec<[CompiledHeadInfo<E>; 4]>,
}

impl<E> Default for CompiledTraverseInfo<E> {
    fn default() -> Self {
        Self {
            heads: Default::default(),
        }
    }
}

impl<E> CompiledTraverseInfo<E>
where
    E: Clone,
{
    pub fn remove_empty_heads(&mut self) {
        let mut skip_indices = vec![];

        for (i, head) in self.heads.iter().enumerate() {
            if head.indices.len() == 0 {
                skip_indices.push(i);
            }
        }

        if skip_indices.len() > 0 {
            let mut skip_indices = skip_indices.into_iter().peekable();

            let mut new_heads = smallvec![];

            for (i, head) in self.heads.iter().enumerate() {
                if skip_indices.peek().map(|skip| i != *skip).unwrap_or(true) {
                    new_heads.push(head.clone());
                } else {
                    skip_indices.next();
                }
            }

            self.heads = new_heads;
        }
    }
}
