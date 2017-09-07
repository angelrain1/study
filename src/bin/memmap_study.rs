extern crate memmap;

use std::io::Result;

use std::io::prelude::*;

use std::fs::File;

use memmap::{Mmap, Protection};

fn run() -> Result<()> {
    {
        let mut f = File::create("content.txt")?;
        f.write_all(b"My hovercraft      l  o      !")?;
    }

    let f = File::open("content.txt")?;

    let map = Mmap::open(&f, Protection::Read)?;

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    // This is only safe if no other code is modifying the file at the same time
    unsafe {
        let map = map.as_slice();
        assert_eq!(&map[3..13], b"hovercraft");
        // I'm using an iterator here to change indexes to bytes
        let random_bytes: Vec<u8> = random_indexes.iter()
            .map(|&idx| map[idx])
            .collect();
        assert_eq!(&random_bytes[..], b"My loaf!");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}