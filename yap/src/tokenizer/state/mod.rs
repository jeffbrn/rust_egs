pub mod transition;

use std::sync::atomic::{AtomicI32, Ordering};
use crate::tokenizer::state::transition::transitions::Transition;

pub struct IdMgr {
  next_id : AtomicI32,
}

pub struct State<'a> {
  id : i32,
  root: Option<&'a State<'a>>,
  transitions: Vec<Transition<'a>>,
  pub emit_token : Option<i32>
}

impl<'a> State<'a> {
  pub fn context() -> IdMgr {
    IdMgr { next_id: AtomicI32::new(1) }
  }

  pub fn new(context: &mut IdMgr, root: Option<&'a State<'a>>) -> State<'a> {
    let id = context.next_id.fetch_add(1, Ordering::SeqCst);
    match root {
        None => if id > 1 {
          panic!("Non root state must include root");
        },
        _ => (),
    }
    State { id, root, transitions: Vec::new(), emit_token : None }
  }

  pub fn is_root_state(&self) -> bool {
    self.id == 1
  }

  pub fn walk(&self, ch: char) -> &State {
    if self.is_root_state() && char::is_whitespace(ch) {
      return self
    }
    let trans = self.transitions.iter().find(|&x| x.do_transition(ch));
    match trans {
        Some(t) => t.next,
        None => match self.root {
            Some(r) => r,
            None => self,
        },
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tokenizer::state::State;

  #[test]
  fn root_checks() {
    let mut context = State::context();
    let root = State::new(&mut context, None);
    assert_eq!(1, root.id);
    assert!(root.is_root_state());
    let _st = State::new(&mut context, Some(&root));
    assert_eq!(2, _st.id);
    assert!(!_st.is_root_state());
  }

  #[test]
  #[should_panic]
  fn dont_pass_root_state() {
    let mut context = State::context();
    let _root = State::new(&mut context, None);
    // every state after the first should pass the initial state in the constructor
    let _st = State::new(&mut context, None);
  }

  #[test]
  fn check_constructor() {
    let mut context = State::context();
    let root = State::new(&mut context, None);
    assert!(root.emit_token.is_none());
    assert!(root.root.is_none());
    assert_eq!(root.transitions.len(), 0);
  }
}