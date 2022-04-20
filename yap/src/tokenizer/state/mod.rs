use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct State {
  pub emit_token : Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Transition {
  min: char,
  max: char,
  exception: Option<char>,
  next_state: usize,
}

pub struct LexerStates {
  states : Vec<State>,
  transitions : HashMap<usize, Vec<Transition>>,
}

impl LexerStates {
  pub fn new() -> LexerStates {
    LexerStates { states: Vec::new(), transitions: HashMap::new() }
  }

  pub fn add_state(&mut self, token: Option<usize>) -> usize {
    let idx = self.states.len();
    self.states.push(State { emit_token: token });
    idx
  }

  pub fn add_transition(&mut self, from_state: usize, min: char, max: char, ex: Option<char>, next: usize) {
    let default: Vec<Transition> = Vec::new();
    let v = self.transitions.entry(from_state).or_insert(default);
    v.push(Transition { min, max, exception: ex, next_state: next });
  }
}

impl Default for LexerStates {
  fn default() -> Self {
    Self::new()
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
    assert_eq!(states.states.len(), 2);
  }

  #[test]
  fn add_transitions() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    let next = states.add_state(Some(4));
    states.add_transition(root, '=', '=', None, root);
    states.add_transition(root, 'a', 'z', None, next);
  }
}