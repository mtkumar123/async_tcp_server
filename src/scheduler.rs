use std::any::Any;

use crate::{
    future::Future,
    handler::SocketHandler,
    waker::{LocalWaker, Waker},
};

trait Scheduler {
    type Future: Future;
    fn spawn(&mut self, task: Self::Future);
    fn run(&mut self);
}

struct LocalScheduler<'a> {
    runnable: Vec<&'a Box<dyn Future<Waker = LocalWaker>>>,
}

impl<'a> Scheduler for LocalScheduler<'a> {
    type Future = SocketHandler;

    fn spawn(&mut self, task: Self::Future) {
        self.runnable.push(&Box::new(task));
    }

    fn run(&mut self) {
        // for all the futures in runnable poll them
        // then wait on the reactor to update
        for run in self.runnable.iter() {
            run.poll(LocalWaker(Box::new(|| self.runnable.push(run))));
        }
    }
}
