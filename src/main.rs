mod handler;
mod future;
mod reactor;
mod scheduler;
mod waker;
mod side;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::OnceLock;

use future::Future;
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use scheduler::LocalScheduler;

use crate::waker::LocalWaker;
use crate::{scheduler::Scheduler, waker::Waker};

use crate::reactor::{LocalReactor, Reactor};

enum Main {
    Start
}

const SERVER: Token = Token(0);

impl Future for Main {
    type Waker = LocalWaker;
    type Reactor = LocalReactor;

    fn poll(&self, reactor: &mut Self::Reactor, waker: Self::Waker) {
        match self {
            Main::Start => {
                let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7777));
                let mut listener = TcpListener::bind(addr).unwrap();
                reactor.add_event(SERVER, waker, listener);
            },
        }

    }
}

fn main() {
    let mut scheduler: LocalScheduler = LocalScheduler::new();
    scheduler.spawn(Main::Start);
    scheduler.run();
}
