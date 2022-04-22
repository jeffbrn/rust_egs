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

impl Transition {
  pub fn check(&self, ch: char) -> Option<usize> {
    if let Some(ex) = self.exception {
      if ch == ex {
        return None;
      }
    }
    if self.min <= ch && ch <= self.max {
      return Some(self.next_state);
    }
    None
  }

  pub fn is_equal(&self, min: char, max: char, ex: Option<char>) -> bool {
    self.min == min && self.max == max && self.exception == ex
  }
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
    if from_state >= self.states.len() {
      panic!("invalid from state");
    }
    if next >= self.states.len() {
      panic!("invalid next state");
    }
    if let Some(ch) = ex {
      if ch <= min || ch >= max {
        panic!("exception must be between limits");
      }
    }

    let default: Vec<Transition> = Vec::new();
    let v = self.transitions.entry(from_state).or_insert(default);
    let t = Transition { min, max, exception: ex, next_state: next };
    v.push(t);
  }

  pub fn walk_states(&self, from_state: usize, ch: char) -> Option<usize> {
    if from_state == 0 && char::is_whitespace(ch) {
      return Some(0);
    }
    let trans = self.transitions.get(&from_state);
    let tlist = match trans {
        Some(v) => v,
        None => return None,
    };

    let trans = tlist.iter().find(|&x| x.check(ch) != None);
    match trans {
        Some(t) => Some(t.next_state),
        None => None,
    }
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

  fn walk_setup() -> LexerStates {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    let next = states.add_state(Some(4));
    states.add_state(None);
    states.add_transition(root, '=', '=', None, root);
    states.add_transition(root, 'a', 'z', None, next);
    states.add_transition(next, '0', '9', None, next);
    states
  }

  #[test]
  fn walk_whitespace() {
    let states = walk_setup();
    let result = states.walk_states(0, '\t');
    assert_eq!(result, Some(0));
    let result = states.walk_states(1, ' ');
    assert_eq!(result, None);
  }

  #[test]
  fn walk_states() {
    let states = walk_setup();
    let next = states.walk_states(0, '=');
    assert_eq!(next, Some(0));
    let next = states.walk_states(0, 'c');
    assert_eq!(next, Some(1));
    let next = states.walk_states(1, '3');
    assert_eq!(next, Some(1));
  }

  #[test]
  fn walk_failures() {
    let states = walk_setup();
    let next = states.walk_states(2, 'a');
    assert_eq!(next, None);
    let next = states.walk_states(0, 'S');
    assert_eq!(next, None);
  }

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

  #[test]
  #[should_panic]
  fn add_transition_invalid_from() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    states.add_transition(5, '=', '=', None, root);
  }

  #[test]
  #[should_panic]
  fn add_transition_invalid_next() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    states.add_transition(root, '=', '=', None, 3);
  }

  #[test]
  #[should_panic]
  fn add_transition_invalid_exception1() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    states.add_transition(root, 'a', 'z', Some('a'), root);
  }

  #[test]
  #[should_panic]
  fn add_transition_invalid_exception2() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    states.add_transition(root, 'a', 'z', Some('z'), root);
  }

  #[test]
  #[should_panic]
  fn add_transition_invalid_exception3() {
    let mut states = LexerStates::new();
    let root = states.add_state(None);
    states.add_transition(root, 'a', 'z', Some('A'), root);
  }

  #[test]
  fn check_transition_no_except() {
    let t = Transition { min: 'a', max: 'z', exception: None, next_state: 0};
    let result = t.check('a');
    assert_eq!(result, Some(0));
    let result = t.check('z');
    assert_eq!(result, Some(0));
    let result = t.check('m');
    assert_eq!(result, Some(0));
    let result = t.check('A');
    assert_eq!(result, None);
  }

  #[test]
  fn check_transition_except() {
    let t = Transition { min: 'a', max: 'z', exception: Some('x'), next_state: 0};
    let result = t.check('a');
    assert_eq!(result, Some(0));
    let result = t.check('z');
    assert_eq!(result, Some(0));
    let result = t.check('m');
    assert_eq!(result, Some(0));
    let result = t.check('A');
    assert_eq!(result, None);
    let result = t.check('x');
    assert_eq!(result, None);
  }
}