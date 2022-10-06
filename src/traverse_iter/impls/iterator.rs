use crate::{AffectHeads, Head, TraverseIter};

impl<'a, 'b, T, E: 'b> Iterator for TraverseIter<'a, 'b, T, E>
where
    T: AffectHeads<E>,
    E: Clone
{
    type Item = Box<[(&'a T, &'b Head<E>)]>;

    fn next(&mut self) -> Option<Self::Item> {
        let items = self.traverse.next();

        items.map(|items| {
            items.into_iter().enumerate().map(|(i, item)| {
                (*item, self.traverse.heads().get(i).unwrap())
            }).collect()
        })
    }
}