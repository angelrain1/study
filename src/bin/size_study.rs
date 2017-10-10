#![feature(const_fn, const_size_of)]

trait SizeTrait {
    const SIZE: usize;
}

struct A;

impl<T> SizeTrait for T {
    const SIZE: usize = std::mem::size_of::<T>();
}

//
//impl SizeTrait for A {
//    const SIZE: usize = std::mem::size_of::<A>();
//}

fn main() {
    println!("the ID of Struct is: {}", u32::SIZE);
    println!("the ID of Struct is: {}", A::SIZE);
}