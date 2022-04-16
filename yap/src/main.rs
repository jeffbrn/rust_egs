pub mod tokenizer;

use crate::tokenizer::state;

fn main() {
    println!("Hello, world!");
    let mut context = state::State::context();
    let root = state::State::new(&mut context, None, &[]);
    let _st = state::State::new(&mut context, Some(&root), &[]);
}
