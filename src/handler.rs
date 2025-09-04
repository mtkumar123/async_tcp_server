use mio::net::TcpStream;

use crate::{future::Future, waker::LocalWaker};

enum SocketHandlerState {
    Init,  // nothing started yet?
    Read,  // have to remember how much was read?
    Write, // have to remember how much was written?
    Closed,
}
pub struct SocketHandler {
    // have a state machine here
    // so we know where we are in the polling phase?
    state: SocketHandlerState,
    connection: TcpStream,
}

impl SocketHandler {
    fn new(connection: TcpStream) -> Self {
        SocketHandler {
            state: SocketHandlerState::Init,
            connection: connection,
        }
    }
}

impl Future for SocketHandler {
    type Waker = LocalWaker;
    fn poll(&self, waker: Self::Waker) {
        // start work but suspend when I need to block
        // offload that work to the reactor?
        // pass the waker and object to epoll on to the reactor - so it can wake up when it is ready?
        match self.state {
            SocketHandlerState::Init => {}
            SocketHandlerState::Read => {}
            SocketHandlerState::Write => {}
            SocketHandlerState::Closed => {}
        }
    }
}
