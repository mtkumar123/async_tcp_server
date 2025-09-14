use crate::{reactor::Reactor, waker::Waker};

pub trait Future {
    type Waker: Waker;
    type Reactor: Reactor;

    fn poll(&mut self, reactor: &mut Self::Reactor, waker: Self::Waker);
}