
pub mod affect_traverse;
pub mod queue;
pub mod traverse;
pub mod traverse_iter;

pub use affect_traverse::*;
pub use queue::*;
pub use traverse::*;
pub use traverse_iter::*;

pub fn add_head<E>(state: HeadsState<E>) -> HeadsState<E>
where
    E: Clone,
{
    let head = state.heads[state.head_index].clone();
    for head in state.heads.iter_mut().skip(state.head_index) {
        head.index += 1;
    }
    state.heads.insert(state.head_index, head);

    state
}

pub fn add_effect<E>(effect: E, state: HeadsState<E>) -> HeadsState<E>
where
    E: Clone,
{
    state.heads[state.head_index].effects.push(effect);
    state
}
