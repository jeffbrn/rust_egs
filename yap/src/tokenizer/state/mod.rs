use std::sync::atomic::{AtomicI32, Ordering};

pub struct IdMgr {
  next_id : AtomicI32,
}

pub struct State<'a> {
  pub id : i32,
  pub root: Option<&'a State<'a>>,
  //pub transitions: Vec<Transition<'a>>,
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
    State { id, root, /*transitions: Vec::new(),*/ emit_token : None }
  }

  pub fn is_root_state(&self) -> bool {
    self.id == 1
  }
/*
  pub fn walk(&self, ch: char) -> &State {
    if self.is_root_state() && char::is_whitespace(ch) {
      return self
    }
    let trans = self.transitions.iter().find(|&x| x.do_transition(ch));
    match trans {
        Some(t) => t.next,
        None => self
    }
  }
*/
}
