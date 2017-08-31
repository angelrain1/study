#![feature(proc_macro, conservative_impl_trait, generators, generator_trait)]

fn returns_closure() -> impl FnMut(i32) -> i32 {
    let mut a = [0; 4];
    move |x| {
        a[x as usize] += 1;
        x + a[x as usize]
    }
}

extern crate futures_await as futures;

use futures::prelude::*;

#[async]
fn foo() -> Result<i32, i32> {
    Ok(1 + await!(bar())?)
}

#[async]
fn bar() -> Result<i32, i32> {
    Ok(2)
}

use std::ops::Generator;

fn yield_during_range_iter() {
    // Should be OK.
    let mut b = || {
        let a = 0u32;
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
        println!("haha");
        yield a;
    };
    println!("size of vec:{}", std::mem::size_of::<Vec<i32>>());
    println!("size of vec:{}", std::mem::size_of::<std::slice::Iter<i32>>());
    println!("size of gen:{}", std::mem::size_of_val(&b));
    println!("{:?}", b.resume());
    println!("{:?}", b.resume());
    println!("{:?}", b.resume());
    println!("{:?}", b.resume());
}

fn main() {
    yield_during_range_iter();

    let r = foo();
    assert_eq!(r.wait(), Ok(3));

    let mut f = returns_closure();

    println!("size:{}", std::mem::size_of_val(&f));

    println!("{}", f(0));
    println!("{}", f(1));
    println!("{}", f(2));
    println!("{}", f(3));


    let mut a = 1u32;
    let mut b = [1u8; 100];
    println!("read b[0]:{}", b[0]);

    {
        let mut f = move || {
            println!("read b[0]:{}", b[0]);
            println!("write b[0]");
            b[0] += 1;
            println!("read b[0]:{}", b[0]);
            a += 1;
            a
        };

        println!("{}bytes", std::mem::size_of_val(&f));

        println!("a:{}", f());
        println!("a:{}", f());
        println!("a:{}", f());
        println!("a:{}", f());
        println!("a:{}", f());
    }

    println!("a:{}", a);
    println!("read b[0]:{}", b[0]);
}