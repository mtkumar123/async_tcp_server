use std::rc::Rc;

pub trait Waker {
    fn wake(self);
}

pub struct LocalWaker(pub Box<dyn FnOnce()>);

impl Waker for LocalWaker {
    fn wake(self) {
        self.0()
    }
}
