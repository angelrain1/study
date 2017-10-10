extern crate mio;
extern crate miow;
extern crate winapi;

use mio::{Evented, Poll, PollOpt, Ready, Token};

use mio::windows::{Binding};

use std::os::windows::prelude::*;

use std::fs::{File as StdFile, OpenOptions};

use std::io;

struct File {
    file: StdFile,
    bind: Binding,
}
/**/
impl File {
    fn new(raw: StdFile) -> File {
        File {
            file: raw,
            bind: Binding::new(),
        }
    }
}

impl Evented for File {
    fn register(&self, poll: &Poll, token: Token, _interest: Ready, _opts: PollOpt) -> io::Result<()> {
        unsafe {
            self.bind.register_handle(&self.file, token, poll)?;
        }
        Ok(())
    }

    fn reregister(&self, poll: &Poll, token: Token, _interest: Ready, _opts: PollOpt) -> io::Result<()> {
        unsafe {
            self.bind.reregister_handle(&self.file, token, poll)?;
        }
        Ok(())
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        unsafe {
            self.bind.deregister_handle(&self.file, poll)?;
        }
        Ok(())
    }
}

fn main() {
    use mio::{Events, Poll};

    let mut events = Events::with_capacity(1024);
    let poll = Poll::new().unwrap();

    assert_eq!(0, events.len());

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(winapi::FILE_FLAG_OVERLAPPED)
        .open("1.txt")
        .expect("");

    let f = File::new(file);

    poll.register(&f, Token(1), Ready::writable() | Ready::readable(), PollOpt::edge()).expect("");

    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    std::thread::spawn(move || {
        {
            if let Err(e) = FileExt::seek_write(&f.file, b"1234546526", 0) {
                println!("e:{},kind:{:?}", e, e.kind());
                println!("{}", winapi::ERROR_IO_PENDING);
            }
        }

        rx.recv().expect("");
    });


    // Register `Evented` handles with `poll`
    let u = poll.poll(&mut events, None).expect("");

    println!("haha:{}", u);

    for event in &events {
        println!("event={:?}", event);
    }

    tx.send(()).expect("");

    println!("hehe");
}