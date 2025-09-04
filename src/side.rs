use futures::executor::LocalPool;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::marker::PhantomPinned;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

// fn would_block(err: &Error) -> bool {
//     err.kind() == ErrorKind::WouldBlock
// }

// fn main() -> Result<()> {
//     let mut poller = Poll::new()?;
//     let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7777));
//     let mut listener = TcpListener::bind(addr)?;
//     let mut events = Events::with_capacity(100);
//     const SERVER: Token = Token(0);
//     poller
//         .registry()
//         .register(&mut listener, SERVER, Interest::READABLE)?;
//     loop {
//         poller.poll(&mut evenngts, Some(Duration::from_millis(100)))?;
//         for event in events.iter() {
//             match event.token() {
//                 SERVER => loop {
//                     match listener.accept() {
//                         Ok((_connection, address)) => {
//                             println!("Connection from {}", address)
//                         }
//                         Err(ref err) if would_block(err) => break,
//                         Err(err) => {
//                             println!("Error is {:?}", err)
//                         }
//                     }
//                 },
//                 _ => {}
//             }
//         }
//     }
// }

struct Bad {
    data: String,
    ptr: *const String,
}

struct Good {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

fn main() {
    let mut b = Bad {
        data: String::from("Hello World"),
        ptr: std::ptr::null(),
    };
    b.ptr = &b.data;
    let b2 = b;
    // println!("{:?}", *b2.ptr); compiler error since b has moved to b2.
    // b pointer points to where b.data used to be, but that has been moved to b2.data
    // b pointer is a null reference
}
