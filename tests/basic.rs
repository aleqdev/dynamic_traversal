use std::fmt::Debug;

use dynamic_traversal::{
    AffectCompilation, CompilationState, Compile, GeneralCompilationStateExt, TraverseWith,
};

#[derive(Debug)]
struct I(i32);

impl AffectCompilation<isize> for I {
    fn affect(&self, state: CompilationState<isize>) -> CompilationState<isize> {
        state
            .yield_element()
            .when(self.0 % 4 == 0, |state| state.add_heads(1).advance_head(1))
            .no_otherwise()
            .advance_head(1)
            .advance_index(1)
    }
}

#[test]
fn test() {
    let mut c = vec![];

    c.extend((0..10).map(I));

    let mut comp = c.iter().compile();
    comp.remove_empty_heads();

    for chunk in c.traverse_with(&comp) {
        for element in chunk.iter() {
            print!("{: <16} ", format!("{:?}", element));
        }
        println!();
    }
}
