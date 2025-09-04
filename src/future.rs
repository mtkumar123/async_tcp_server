use crate::waker::Waker;

pub trait Future {
    type Waker: Waker;
    fn poll(&self, waker: Self::Waker);
}
