use super::subject::Subject;
use std::sync::Arc;

use super::traits::Observer;

#[allow(dead_code)]
pub struct MyObserver {
    name: String,
    id: i32,
}

impl MyObserver {
    #[allow(dead_code)]
    pub fn new_state(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            id: 0,
        })
    }

    #[allow(dead_code)]
    pub fn new(id: i32) -> Arc<Self> {
        Arc::new(Self {
            name: "initial state".to_string(),
            id,
        })
    }
}

impl Observer for MyObserver {
    type Subject = Subject;

    fn observe(&self, subject: &Self::Subject) -> i32 {
        println!(
            "Observer {}: Subject state changed to '{}'",
            self.name,
            subject.state()
        );
        self.id
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::super::subject::Subject;
    #[allow(unused_imports)]
    use super::super::traits::Observable;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_observer_pattern() {
        let mut subject = Subject::new(2);

        let observer1 = MyObserver::new(1);
        let observer2 = MyObserver::new(4);

        subject.attach(observer1.clone());
        subject.attach(observer2.clone());

        let result = subject.update(); // Notify observers
        assert_eq!(result, 10);

        subject.detach(observer1.clone());

        let result = subject.update(); // Notify remaining observers
        assert_eq!(result, 8);
    }
}
