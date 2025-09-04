pub trait Waker {
    fn wake(&self);
}

pub struct LocalWaker(pub Box<dyn Fn()>);

impl Waker for LocalWaker {
    fn wake(&self) {
        self.0()
    }
}
