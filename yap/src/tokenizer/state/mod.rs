use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct State {
  pub emit_token : Option<i32>,
}

pub struct Transition {
  min: char,
  max: char,
  exception: Option<char>,
  next_state: i32,
}

pub struct LexerStates {
  pub states : Vec<State>,
  pub transitions : HashMap<i32, Vec<Transition>>,
}

impl LexerStates {
  pub fn new() -> LexerStates {
    LexerStates { states: Vec::new(), transitions: HashMap::new() }
  }

  pub fn add_state(&mut self, token: Option<i32>) -> usize {
    let idx = self.states.len();
    self.states.push(State { emit_token: token });
    idx
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_states() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    assert_eq!(root, 0);
    let next = states.add_state(Some(4));
    assert_eq!(next, 1);
    let st = states.states[next];
    assert_eq!(st.emit_token, Some(4));
  }
}