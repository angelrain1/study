trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

//impl Bar for Baz {
//    fn f(&self) { println!("Baz’s impl of Bar"); }
//}
//
//impl Baz {
//    fn f(&self) { println!("Baz's impl"); }
//}

fn main() {
    let t: &mut i32 = &mut 0;
    foo(t as &i32);

    let b = Baz;
    b.f();
}