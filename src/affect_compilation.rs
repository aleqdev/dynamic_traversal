
use enum_dispatch::enum_dispatch;

use crate::Heads;

pub struct HeadsState<'a, E> {
    pub heads: &'a mut Heads<E>,
    pub head_index: usize,
}

#[enum_dispatch]
pub trait AffectHeads<E> {
    fn affect<'a>(&self, input: HeadsState<'a, E>) -> HeadsState<'a, E>;
}

impl<E, F> AffectHeads<E> for F
    where
        F: for<'a> Fn(HeadsState<'a, E>) -> HeadsState<'a, E>,
{
    fn affect<'a>(&self, input: HeadsState<'a, E>) -> HeadsState<'a, E> {
        self(input)
    }
}
