mod handler;
mod future;
mod reactor;
mod scheduler;
mod waker;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use future::Future;
use mio::{net::TcpListener, Token};
use scheduler::LocalScheduler;

use crate::waker::LocalWaker;
use crate::scheduler::Scheduler;

use crate::reactor::{LocalReactor, Reactor};

enum Main {
    Init,
    Listening(Option<TcpListener>)
}

const SERVER: Token = Token(0);

impl Future for Main {
    type Waker = LocalWaker;
    type Reactor = LocalReactor;

    fn poll(&mut self, reactor: &mut Self::Reactor, waker: Self::Waker) {
        match self {
            Main::Init => {
                let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7777));
                let mut listener = TcpListener::bind(addr).unwrap();
                reactor.add_event(SERVER, waker, &mut listener);
                *self = Main::Listening(Some(listener));
            }
            Main::Listening(listener) => {
                let listener = listener.take().unwrap();
                let (stream, addr) = listener.accept().unwrap();
                
                reactor.register_waker(SERVER, waker);
            },
        }

    }
}

fn main() {
    println!("Starting up...");
    let mut scheduler: LocalScheduler = LocalScheduler::new();
    scheduler.spawn(Main::Init);
    scheduler.run();
}
