#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(placement_in_syntax)]
#![feature(placement_new_protocol)]
#![feature(collection_placement)]

struct A(i32);

use std::ops::Placer;

pub struct Stack(Vec<u32>);

impl Default for Stack {
    fn default() -> Self {
        Stack(Vec::new())
    }
}

impl<'a> Placer<u32> for &'a mut Stack {
    type Place = std::vec::PlaceBack<'a, u32>;

    fn make_place(self) -> Self::Place {
        println!("here!");
        self.0.place_back().make_place()
    }
}

fn main() {
    let mut stack = Stack::default();
    &mut stack <- 1u32;
    println!("Doesn't get here!");


    let mut map = std::collections::HashMap::new();

    let k = 8;
    let v = 10;

    map.entry(k) <- v;

    let b = box 1234;

    println!("{}", b);

    let b = Some(Box::new(A(5)));
    match b {
        Some(box ref a) if a.0 < 0 => {
            println!("Box contains negative number {}", a.0);
        },
        Some(box ref a) if a.0 >= 0 => {
            println!("Box contains non-negative number {}", a.0);
        },
        None => {
            println!("No box");
        },
        _ => unreachable!()
    }

    let b = Box::new(5);

    println!("{}", b);


    let mut vec = vec![A(1),A(2)];
    {
        let a = vec.place_back() <- A(3);
        a.0 = 4;
    }
    vec.place_back() <- A(3);

    let x: Result<_, _> = MyBox::place().map(|p| p <- 1); // ....map(|p| p <- 1)
    // with `try { ... ? ... }` this could also be:
    // let x = try { in MyBox::place()? { 1 } }; // ... try { MyBoxPlace()? <- 1 };
    println!("{}", *x.unwrap());

    println!("{}", *func(2).unwrap());
}

fn func(val: i32) -> Result<MyBox<i32>, BadAlloc> {
    let mut x = MyBox::place()? <- val; // ... try!(MyBox::place()) <- val
    *x += 10;
    Ok(x)
}

// implementation of the `in`  procotol:

pub struct MyBoxPlace<T> {
    ptr: *mut T
}
#[derive(Debug)]
pub struct BadAlloc;

impl<T> MyBox<T> {
    pub fn place() -> Result<MyBoxPlace<T>, BadAlloc> {
        let p = unsafe {malloc(mem::size_of::<T>())};
        if p.is_null() {
            Err(BadAlloc)
        } else {
            Ok(MyBoxPlace { ptr: p as *mut T })
        }
    }
}
impl<T> ops::Placer<T> for MyBoxPlace<T> {
    type Place = Self;
    fn make_place(self) -> Self { self }
}
impl<T> ops::Place<T> for MyBoxPlace<T> {
    fn pointer(&mut self) -> *mut T { self.ptr }
}

impl<T> ops::InPlace<T> for MyBoxPlace<T> {
    type Owner = MyBox<T>;
    unsafe fn finalize(self) -> MyBox<T> {
        let p = self.ptr as *const T;
        mem::forget(self);
        MyBox { ptr: p }
    }
}
impl<T> Drop for MyBoxPlace<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut u8);
        }
    }
}


// implementation of the pointer itself

use std::{mem, ops, ptr};

extern {
    fn malloc(x: usize) -> *mut u8;
    fn free(p: *mut u8);
}

/// Custom `Box`
pub struct MyBox<T> {
    ptr: *const T
}

// make `MyBox` behave like a pointer
impl<T> ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {&*self.ptr}
    }
}
impl<T> ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *(self.ptr as *mut T)}
    }
}
// etc.

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop(ptr::read(self.ptr));
            free(self.ptr as *mut u8);
        }
    }
}

