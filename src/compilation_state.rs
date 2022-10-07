use crate::{CompiledTraverseInfo, GeneralCompilationData};

#[derive(Debug, Clone)]
pub struct CompilationState<E, D = GeneralCompilationData> {
    pub result: CompiledTraverseInfo<E>,
    pub data: D,
}

impl<E, D> CompilationState<E, D> {
    pub fn then(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }

    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> When<E, D> {
        let state = if cond { f(self) } else { self };
        When { state, cond }
    }
}

pub type GeneralCompilationState<E> = CompilationState<E, GeneralCompilationData>;

#[derive(Debug)]
pub struct When<E, D> {
    pub state: CompilationState<E, D>,
    pub cond: bool,
}

impl<E, D> Into<CompilationState<E, D>> for When<E, D> {
    fn into(self) -> CompilationState<E, D> {
        self.state
    }
}

impl<E, D> When<E, D> {
    pub fn otherwise(
        self,
        f: impl FnOnce(CompilationState<E, D>) -> CompilationState<E, D>,
    ) -> CompilationState<E, D> {
        if self.cond {
            self.into()
        } else {
            f(self.into())
        }
    }

    pub fn no_otherwise(self) -> CompilationState<E, D> {
        self.into()
    }
}
