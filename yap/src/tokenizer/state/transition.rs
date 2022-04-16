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
      let err = match ex {
          None => false,
          Some(ch) => ch == min || ch == max,
      };
      if err {
        panic!("Exception char cannot be the same as the valid char endpoints");
      }
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
    use super::*;

    fn setup<'a>() -> State<'a> {
      let mut context = State::context();
      return State::new(&mut context, None, &[]);
    }
    
    #[test]
    fn check_constructor() {
      let root = setup();
      let t = Transition::new('a', 'z', None, &root);
      assert_eq!(t.next.id, root.id);
    }

    #[test]
    fn check_single_char_transition() {
      let root = setup();
      let t =  Transition::new('=', '=', None, &root);
      assert!(t.do_transition('='));
      assert!(!t.do_transition('a'));
    }

    #[test]
    #[should_panic]
    fn check_bad_transition() {
      let root = setup();
      let _t =  Transition::new('=', '=', Some('='), &root);
    }

    #[test]
    fn check_range_transition() {
      let root = setup();
      let t =  Transition::new('a', 'z', None, &root);
      assert!(t.do_transition('z'));
      assert!(!t.do_transition('A'));
    }

    #[test]
    fn check_range_transition_with_excep() {
      let root = setup();
      let t =  Transition::new('a', 'z', Some('d'), &root);
      assert!(t.do_transition('b'));
      assert!(!t.do_transition('d'));
    }

    #[test]
    fn check_is_same_excep() {
      let root = setup();
      let t =  Transition::new('a', 'z', Some('d'), &root);
      assert!(t.is_same('a', 'z', Some('d')));
      assert!(!t.is_same('b', 'z', Some('d')));
      assert!(!t.is_same('a', 'A', Some('d')));
      assert!(!t.is_same('a', 'z', Some('e')));
      assert!(!t.is_same('a', 'z', None));
    }

    #[test]
    fn check_is_same() {
      let root = setup();
      let t =  Transition::new('=', '=', None, &root);
      assert!(t.is_same('=', '=', None));
      assert!(!t.is_same('=', '=', Some('d')));
      assert!(!t.is_same('a', 'A', None));
    }
  }
}
