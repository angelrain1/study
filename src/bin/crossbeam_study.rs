extern crate crossbeam;

use std::sync::mpsc::*;

fn main() {
    let mut a: u32 = 1;

    // outer immutable borrow once
    println!("outer:{:p},{}", &a, a);

    crossbeam::scope(|scope| {
        let (tx, rx) = channel();

        // immutable borrow occurs here until scope end, this is mean submit a work to the worker/thread
        tx.send(&a).expect("");

        //spawn a scoped worker/thread
        let j = scope.spawn(move || {
            //                let ref_a: &mut u32 = &mut a;
            let ref_a = rx.recv().expect("");
            std::thread::sleep(std::time::Duration::from_millis(250));
            println!("inner:{:p},{}", ref_a, *ref_a);
            std::thread::sleep(std::time::Duration::from_millis(250));
//            *ref_a = 2;
            std::thread::sleep(std::time::Duration::from_millis(250));
            println!("inner:{:p},{}", ref_a, *ref_a);
            std::thread::sleep(std::time::Duration::from_millis(250));
        });


        // do something else and this is concurrent
        for _ in 0..21 {
            println!("outer:do something");
            std::thread::sleep(std::time::Duration::from_millis(50));
            // immutable borrow occurs here until scope end, this is mean submit a work to the worker/thread
            println!("outer:{:p},{}", &a, a);
        }

        j.join();
    });

    crossbeam::scope(|scope| {
        let (tx, rx) = channel();

        // mutable borrow occurs here until scope end, this is mean submit a work to the worker/thread
        tx.send(&mut a).expect("");

        //spawn a scoped worker/thread
        let j = scope.spawn(move || {
            //                let ref_a: &mut u32 = &mut a;
            let ref_a = rx.recv().expect("");
            std::thread::sleep(std::time::Duration::from_millis(250));
            println!("inner:{:p},{}", ref_a, *ref_a);
            std::thread::sleep(std::time::Duration::from_millis(250));
            //            *ref_a = 2;
            std::thread::sleep(std::time::Duration::from_millis(250));
            println!("inner:{:p},{}", ref_a, *ref_a);
            std::thread::sleep(std::time::Duration::from_millis(250));
        });

        // immutable borrow occurs here mean is error
        // println!("outer:{:p},{}", &a, a);

        j.join();
    });

    println!("outer:{:p},{}", &a, a);
}
