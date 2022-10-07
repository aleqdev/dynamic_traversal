pub mod affect_compilation;
pub mod compilation_state;
pub mod compile;
pub mod compiled_head_info;
pub mod compiled_traverse_info;
pub mod general_compilation_data;
pub mod traverse;
pub mod traverse_with;

pub use affect_compilation::*;
pub use compilation_state::*;
pub use compile::*;
pub use compiled_head_info::*;
pub use compiled_traverse_info::*;
pub use general_compilation_data::*;
pub use traverse::*;
pub use traverse_with::*;

pub fn yield_element<E>(mut state: GeneralCompilationState<E>) -> GeneralCompilationState<E> {
    state.result.heads[state.data.head_index]
        .indices
        .push(state.data.element_index);

    state
}

pub fn add_heads<E>(
    count: usize,
    mut state: GeneralCompilationState<E>,
) -> GeneralCompilationState<E>
where
    E: Clone,
{
    let head = CompiledHeadInfo {
        offset: state.data.element_index + state.result.heads.len() + 1,
        indices: Default::default(),
        effects: state.result.heads[state.data.head_index].effects.clone(),
    };

    state
        .result
        .heads
        .insert_many(state.data.head_index + 1, (0..count).map(|_| head.clone()));

    state
}

pub fn add_effect<E>(
    effect: E,
    mut state: GeneralCompilationState<E>,
) -> GeneralCompilationState<E> {
    state.result.heads[state.data.head_index]
        .effects
        .push(effect);

    state
}

pub fn advance_index<E>(
    count: usize,
    mut state: GeneralCompilationState<E>,
) -> GeneralCompilationState<E> {
    state.data.element_index += count;

    state
}

pub fn advance_head<E>(
    count: usize,
    mut state: GeneralCompilationState<E>,
) -> GeneralCompilationState<E> {
    state.data.head_index = (state.data.head_index + count) % state.result.heads.len();

    state
}

pub trait GeneralCompilationStateExt<E> {
    fn yield_element(self) -> Self;
    fn add_heads(self, count: usize) -> Self;
    fn add_effect(self, effect: E) -> Self;
    fn advance_index(self, count: usize) -> Self;
    fn advance_head(self, count: usize) -> Self;

    fn yield_element_if(self, cond: bool) -> Self;
    fn add_heads_if(self, cond: bool, count: usize) -> Self;
    fn add_effect_if(self, cond: bool, effect: E) -> Self;
    fn advance_index_if(self, cond: bool, count: usize) -> Self;
    fn advance_head_if(self, cond: bool, count: usize) -> Self;
}

impl<E> GeneralCompilationStateExt<E> for GeneralCompilationState<E>
where
    E: Clone,
{
    fn yield_element(self) -> Self {
        yield_element(self)
    }

    fn add_heads(self, count: usize) -> Self {
        add_heads(count, self)
    }

    fn add_effect(self, effect: E) -> Self {
        add_effect(effect, self)
    }

    fn advance_index(self, count: usize) -> Self {
        advance_index(count, self)
    }

    fn advance_head(self, count: usize) -> Self {
        advance_head(count, self)
    }

    fn yield_element_if(self, cond: bool) -> Self {
        if cond {
            self.yield_element()
        } else {
            self
        }
    }

    fn add_heads_if(self, cond: bool, count: usize) -> Self {
        if cond {
            self.add_heads(count)
        } else {
            self
        }
    }

    fn add_effect_if(self, cond: bool, effect: E) -> Self {
        if cond {
            self.add_effect(effect)
        } else {
            self
        }
    }

    fn advance_index_if(self, cond: bool, count: usize) -> Self {
        if cond {
            self.advance_index(count)
        } else {
            self
        }
    }

    fn advance_head_if(self, cond: bool, count: usize) -> Self {
        if cond {
            self.advance_head(count)
        } else {
            self
        }
    }
}
