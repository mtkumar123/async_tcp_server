use std::{cell::RefCell, rc::Rc};


pub trait Waker {
    fn wake(self);

    // looks like I need to implement a wake_by_ref somehow
    // or actually use Rc<Self> here
}

pub struct LocalWaker(pub Box<dyn FnOnce()>);

impl Waker for LocalWaker {
    fn wake(self) {
        self.0()
    }
}