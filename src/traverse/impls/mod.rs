pub mod from_queue;
pub mod iterator;

pub use from_queue::*;
pub use iterator::*;

use crate::{AffectHeads, DynamicQueue, Head, Heads, HeadsState, Traverse};

impl<'a, T, E> Traverse<'a, T, E>
where
    T: AffectHeads<E>,
    E: Clone,
{
    pub fn queue(&self) -> &DynamicQueue<T> {
        &self.queue
    }

    pub fn heads(&self) -> &Heads<E> {
        &self.heads
    }

    pub fn heads_mut(&mut self) -> &mut Heads<E> {
        &mut self.heads
    }

    pub fn next(&mut self) -> Option<Box<[(&'a T, &'_ Head<E>)]>> {
        let len = self.heads.len();

        let mut output = Vec::with_capacity(self.heads.len());

        let mut j = 0;
        for _ in 0..len {
            let head = &self.heads[j];

            if head.index >= self.queue.len() {
                break;
            }

            let item = &self.queue[head.index];

            let input = HeadsState {
                heads: &mut self.heads,
                head_index: j,
            };

            let state = item.affect(input);
            j = state.head_index;

            output.push(item);

            j += 1;
        }

        for i in 0..self.heads.len() {
            self.heads[i].index = self.heads[i].index + len;
        }

        match output.len() {
            0 => None,
            _ => Some(
                output.into_iter().zip(self.heads.iter()).collect::<Box<[_]>>()
            )
        }
    }
}
