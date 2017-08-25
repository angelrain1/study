#![feature(box_syntax)]
#![feature(box_heap)]
#![feature(placement_in_syntax)]
#![feature(placement_new_protocol)]
#![feature(collection_placement)]

struct A(u32);

use std::boxed::HEAP;

fn main() {
    let mut map = std::collections::HashMap::new();

    let k = 8;
    let v = 10;

    map.entry(k) <- v;

    let b = box 1234;

    let b = Box::new(5);

    println!("{}", b);


    let mut vec = vec![A(1),A(2)];
    {
        let a = (vec.place_back() <- A(3));
        a.0 = 4;
    }
    let b = (vec.place_back() <- A(3));
}