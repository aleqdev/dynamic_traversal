
use std::fmt::Debug;

use dynamic_queue::{add_head, AffectHeads, HeadsState, add_effect, DynamicQueue, Head, Traverse};

#[derive(Debug)]
struct I(i32);

impl AffectHeads<i32> for I {
    fn affect<'a>(&self, input: HeadsState<'a, i32>) -> HeadsState<'a, i32> {
        add_effect(self.0, add_head(input))
    }
}

struct Iter<'a, 'b, T, E>
where
    T: AffectHeads<E>
{
    traverse: &'b mut Traverse<'a, T, E>
}

impl<'a, 'b, T, E> Iterator for Iter<'a, 'b, T, E>
where
    T: AffectHeads<E>,
    E: Clone
{
    type Item = Box<[(&'a T, &'b Head<E>)]>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traverse.next()
    }
}

fn iterate<'a, T, E>(queue: &'a DynamicQueue<T>) -> impl Iterator + 'a
where
    T: AffectHeads<E>,
    E: Clone + 'a
{
    Iter {
        traverse: &mut queue.iter()
    }
}

#[test]
fn test() {
    use dynamic_queue::*;

    let mut c = DynamicQueue::default();

    c.extend((0..10).map(I));

    println!("{:?}", c);

    let mut t = c.iter();
    while let Some(i) = t.next() {
        println!("{:?}", i);
    }
}
