extern crate mio;

fn main() {
    use mio::{Events, Poll};
    use std::time::Duration;

    let mut events = Events::with_capacity(1024);
    let poll = Poll::new().unwrap();

    assert_eq!(0, events.len());

    // Register `Evented` handles with `poll`
    poll.poll(&mut events, Some(Duration::from_millis(100))).unwrap();

    std::thread::sleep(Duration::from_millis(100));

    println!("haha");

    for event in &events {
        println!("event={:?}", event);
    }

    println!("hehe");

}