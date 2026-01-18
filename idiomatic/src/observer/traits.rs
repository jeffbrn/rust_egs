#[allow(dead_code)]
pub trait Observer {
    type Subject;
    #[allow(dead_code)]
    fn observe(&self, subject: &Self::Subject) -> i32;
}

#[allow(dead_code)]
pub trait Observable {
    type Observer;
    fn update(&self) -> i32;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}
