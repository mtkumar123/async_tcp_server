use crate::waker::{LocalWaker, Waker};
use mio::{event, Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub trait Reactor {
    type Id: Eq + Hash;
    type Waker: Waker;

    fn wait(&mut self);
    fn add_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, waker: Self::Waker, event: &mut T);
    fn register_waker(&mut self, event_id: Self::Id, waker: Self::Waker);
    fn remove_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, event: &mut T);
}

pub struct LocalReactor {
    fd_to_waker: HashMap<Token, LocalWaker>,
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
                    Some(waker) => {
                        waker.wake();
                    },
                    None => {
                        println!("Token not found in Hashmap! {:?}", event.token())
                    }
                }
        }
    }
    
    fn add_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, waker: Self::Waker, event: &mut T) {
        self.poller.registry().register(
            event,
            event_id,
            Interest::READABLE | Interest::WRITABLE,
        );
        self.fd_to_waker.insert(event_id, waker);
    }

    fn register_waker(&mut self, event_id: Self::Id, waker: Self::Waker) {
        self.fd_to_waker.insert(event_id, waker);
    }
    
    fn remove_event<T: event::Source + 'static>(&mut self, event_id: Self::Id, event: &mut T) {
        self.fd_to_waker.remove(&event_id).unwrap();
        self.poller.registry().deregister(event);
    }
}
