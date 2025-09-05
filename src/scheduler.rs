use crate::{future::Future, handler::SocketHandler, waker::LocalWaker};

trait Scheduler {
    type Future: Future;
    fn spawn(&mut self, task: Self::Future);
    fn run(&mut self);
}

struct LocalScheduler<'a> {
    runnable: Vec<Box<dyn Future<Waker = LocalWaker<'a>>>>,
    updated: Vec<Box<dyn Future<Waker = LocalWaker<'a>>>>,
}

impl<'a> Scheduler for LocalScheduler<'a> {
    type Future = SocketHandler;

    fn spawn(&mut self, task: Self::Future) {
        self.runnable.push(Box::new(task));
    }

    fn run(&'a mut self) {
        // for all the futures in runnable poll them
        // then wait on the reactor to update
        self.runnable.append(&mut self.updated);
        for run in self.runnable.drain(..) {
            run.poll(LocalWaker(Box::new(|| self.updated.push(run))))
        }
    }
}
