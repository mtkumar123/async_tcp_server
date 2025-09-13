use crate::future::Future;
use crate::waker::{LocalWaker, Waker};
use core::task;
use mio::net::TcpStream;
use mio::{event, Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Read, Write};
use std::os::fd::RawFd;

pub trait Reactor {
    type Id: Eq + Hash;
    type Waker: Waker;

    fn wait(&mut self);
    fn add_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, waker: Self::Waker, event: T);
    fn remove_event(&mut self, event_id: Self::Id);
}

pub struct LocalReactor {
    fd_to_waker: HashMap<Token, (Box<dyn event::Source>, LocalWaker)>,
    poller: Poll,
}

impl LocalReactor {
    pub fn new() -> Self {
        LocalReactor {
            fd_to_waker: HashMap::new(),
            poller: Poll::new().unwrap(),
        }
    }
}

impl Reactor for LocalReactor {
    type Id = Token;
    type Waker = LocalWaker;

    fn wait(&mut self) {
        let mut events = Events::with_capacity(1024);
        self.poller.poll(&mut events, Option::None);
        for event in events.iter() {
            match self.fd_to_waker.remove(&event.token()) {
                Some((_, waker)) => waker.wake(),
                None => {
                    println!("Token not found in Hashmap! {:?}", event.token())
                }
            }
        }
    }
    
    fn add_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, waker: Self::Waker, mut event: T) {
        self.poller.registry().register(
            &mut event,
            event_id,
            Interest::READABLE | Interest::WRITABLE,
        );
        self.fd_to_waker.insert(event_id, (Box::new(event), waker));
    }
    
    fn remove_event(&mut self, event_id: Self::Id) {
        self.fd_to_waker.remove(&event_id);
    }
}
