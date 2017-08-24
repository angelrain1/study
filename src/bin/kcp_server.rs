extern crate futures;
extern crate tokio_core;
extern crate tokio_kcp;
extern crate tokio_io;
extern crate env_logger;

use std::env;
use std::net::SocketAddr;

use futures::future::Future;
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use tokio_io::io::copy;
use tokio_kcp::KcpListener;

use std::net::UdpSocket;

fn main() {
//    let socket = UdpSocket::bind("127.0.0.1:1234").expect("couldn't bind to address");
//
//    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
//
//    for _ in 0..20 {
//        socket.send_to("123123".as_bytes(), addr).expect("sendto failed");
//    }
//
//    let mut vec = vec![];
//
//    println!("hehe");
//    while let Err(e) = socket.recv_from(&mut vec) {
//        println!("{}", e);
//    }
//    println!("haha");


    let _ = env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "192.168.12.82:1234".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = KcpListener::bind(&addr, &handle).unwrap();
    println!("listening on: {}", addr);

    let echo = listener.incoming().for_each(|(stream, addr)| {
        let (reader, writer) = stream.split();
        let amt = copy(reader, writer);
        let msg = amt.then(move |result| {
            match result {
                Ok((amt, ..)) => println!("wrote {} bytes to {}", amt, addr),
                Err(e) => println!("error on {}: {}", addr, e),
            }
            Ok(())
        });

        handle.spawn(msg);

        Ok(())
    });

    core.run(echo).unwrap();
}
