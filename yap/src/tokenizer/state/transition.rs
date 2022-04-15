pub mod transitions {
  use crate::state;

  pub struct Transition<'a> {
    min: char,
    max: char,
    exception: Option<char>,
    pub next: &'a state::State<'a>,
  }

  impl<'a> Transition<'a> {
    pub fn new(min: char, max: char, ex: Option<char>, next: &'a state::State) -> Transition<'a> {
      Transition { min, max, exception: ex, next }
    }

    pub fn do_transition(&self, ch: char) -> bool {
      match self.exception {
        Some(c) => ch != c && self.min <= ch && ch <= self.max,
        None => self.min <= ch && ch <= self.max,
      }
    }

    pub fn is_same(&self, min: char, max: char, ex: Option<char>) -> bool {
      self.min == min && self.max == max && self.exception == ex
    }
  }

  #[cfg(test)]
  mod tests {
    use crate::tokenizer::state::State;
    use crate::tokenizer::state::Transition;

    #[test]
    fn check_constructor() {
      let mut context = State::context();
      let root = State::new(&mut context, None);
      let t = Transition::new('a', 'z', None, &root);
      assert_eq!(t.next.id, root.id);
    }
  }
}
