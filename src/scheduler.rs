use crate::{future::Future, reactor::{LocalReactor, Reactor}, waker::{LocalWaker, Waker}};
use std::{cell::RefCell, marker::PhantomData, rc::Rc, sync::mpsc};

pub trait Scheduler{
    type Waker: Waker;
    type Reactor: Reactor;
    fn spawn<T: Future<Waker= Self::Waker, Reactor = Self::Reactor> + 'static>(&self, task: T);
    fn run(&mut self);
}

pub struct LocalScheduler {
    sender: mpsc::Sender<Rc<RefCell<dyn Future<Waker = LocalWaker, Reactor = LocalReactor>>>>,
    receiver: mpsc::Receiver<Rc<RefCell<dyn Future<Waker = LocalWaker, Reactor = LocalReactor>>>>,
    reactor: LocalReactor
}

impl LocalScheduler {
    pub fn new() -> LocalScheduler {
        let (tx, rx) = mpsc::channel();
        LocalScheduler { sender: tx, receiver: rx, reactor: LocalReactor::new() }
    }
}

impl Scheduler for LocalScheduler
{
    type Waker = LocalWaker;
    type Reactor = LocalReactor;

    fn spawn<T: Future<Waker= Self::Waker, Reactor = Self::Reactor> + 'static>(&self, task: T) {
        self.sender.send(Rc::new(RefCell::new(task))).unwrap_or_else(|e| println!("Failed to send {:?}", e))
    }

    fn run(&mut self) {
        // for all the futures in runnable poll them
        // then wait on the reactor to update
        loop {
            for task in self.receiver.try_iter() {
                let sender_clone =   self.sender.clone();
                let task_clone = task.clone();
                let wake = move || {
                    sender_clone.send(task_clone);
                };
                task.borrow_mut().poll(&mut self.reactor, LocalWaker(Box::new(wake)));
            }
            // now wait on the reactor
            self.reactor.wait();
        }
        
    }
}
