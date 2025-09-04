use crate::future::Future;
use crate::waker::{LocalWaker, Waker};
use core::task;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Read, Write};
use std::os::fd::RawFd;

trait Reactor {
    type Waker: Waker;
    type Id: Eq + Hash;
    type Stream: Read + Write;

    fn wait(&mut self);
    fn add_task(&mut self, task_id: Self::Id, waker: Self::Waker, connection: Self::Stream);
    fn remove_task(&mut self, task_id: Self::Id);
}

struct LocalReactor {
    fd_to_waker: HashMap<Token, (TcpStream, LocalWaker)>,
    poller: Poll,
}

impl LocalReactor {
    fn new() -> Self {
        LocalReactor {
            fd_to_waker: HashMap::new(),
            poller: Poll::new().unwrap(),
        }
    }
}

impl Reactor for LocalReactor {
    type Waker = LocalWaker;
    type Id = Token;
    type Stream = TcpStream;

    fn add_task(&mut self, task_id: Self::Id, waker: Self::Waker, mut connection: Self::Stream) {
        self.poller.registry().register(
            &mut connection,
            task_id,
            Interest::READABLE | Interest::WRITABLE,
        );
        self.fd_to_waker.insert(task_id, (connection, waker));
    }

    fn remove_task(&mut self, task_id: Self::Id) {
        self.fd_to_waker.remove(&task_id);
    }

    fn wait(&mut self) {
        let mut events = Events::with_capacity(1024);
        self.poller.poll(&mut events, Option::None);
        for event in events.iter() {
            match self.fd_to_waker.get(&event.token()) {
                Some((_, waker)) => waker.wake(),
                None => {
                    println!("Token not found in Hashmap! {:?}", event.token())
                }
            }
        }
    }
}
