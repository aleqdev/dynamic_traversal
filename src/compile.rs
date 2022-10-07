use smallvec::smallvec;

use crate::{
    AffectCompilation, CompilationState, CompiledHeadInfo, CompiledTraverseInfo,
    GeneralCompilationData,
};

pub trait Compile<E, D = GeneralCompilationData>: Iterator {
    fn compile(self) -> CompiledTraverseInfo<E>;
}

impl<Iter, E, D> Compile<E, D> for Iter
where
    Iter: Iterator,
    Iter::Item: AffectCompilation<E, D>,
    D: Default,
{
    fn compile(self) -> CompiledTraverseInfo<E> {
        let mut result = CompiledTraverseInfo {
            heads: smallvec![CompiledHeadInfo::default()],
        };

        let mut data = D::default();

        for el in self {
            let state = el.affect(CompilationState { result, data });

            result = state.result;
            data = state.data;
        }

        result
    }
}
