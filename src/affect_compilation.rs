use crate::{CompilationState, GeneralCompilationData};

pub trait AffectCompilation<E, D = GeneralCompilationData> {
    fn affect(&self, state: CompilationState<E, D>) -> CompilationState<E, D>;
}

impl<'a, T, E, D> AffectCompilation<E, D> for &'a T
where
    T: AffectCompilation<E, D>,
{
    fn affect(&self, state: CompilationState<E, D>) -> CompilationState<E, D> {
        T::affect(self, state)
    }
}

impl<'a, T, E, D> AffectCompilation<E, D> for &'a mut T
where
    T: AffectCompilation<E, D>,
{
    fn affect(&self, state: CompilationState<E, D>) -> CompilationState<E, D> {
        T::affect(self, state)
    }
}
