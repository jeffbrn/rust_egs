use super::traits::{Observable, Observer};
use std::{sync::Arc, sync::Weak};

pub struct Subject {
    #[allow(dead_code)]
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
    #[allow(dead_code)]
    id: i32,
}

impl Subject {
    #[allow(dead_code)]
    pub fn new_state(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.to_string(),
            id: 0,
        }
    }

    #[allow(dead_code)]
    pub fn new(id: i32) -> Self {
        Self {
            observers: vec![],
            state: "initial state".to_string(),
            id,
        }
    }

    #[allow(dead_code)]
    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;

    fn update(&self) -> i32 {
        let sum = self
            .observers
            .iter()
            .flat_map(|o| o.upgrade())
            .map(|obs| obs.observe(self) * self.id)
            .sum();
        sum
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}
