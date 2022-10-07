use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct CompiledHeadInfo<E> {
    pub effects: SmallVec<[E; 4]>,
    pub indices: SmallVec<[usize; 4]>,
    pub offset: usize,
}

impl<E> Default for CompiledHeadInfo<E> {
    fn default() -> Self {
        Self {
            effects: Default::default(),
            indices: Default::default(),
            offset: 0,
        }
    }
}
