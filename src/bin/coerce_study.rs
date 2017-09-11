trait Trait {}

fn foo<X: Trait>(_: X) {}

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

const CONSTANT_STRING: &'static str = "This is a constant string";

const CONSTANT_ARRAY: &'static [u8] = &[0u8; 128];

fn assert_same_type<T: ?Sized>(_: &T, _: &T) {}

fn main() {
    let another_string = "This string is local to the main function";

    let bbb = String::from("akdlfjadlf手动阀乐山大佛科技阿里23");

    println!("{:?}", bbb.chars().size_hint());

    println!("{:?}", bbb.chars().count());

    assert_same_type(CONSTANT_STRING, another_string);

    println!("size of constant string:{}", std::mem::size_of_val(CONSTANT_STRING));
    println!("size of another  string:{}", std::mem::size_of_val(another_string));

    let another_array = &[1u8; 64];

    println!("size of constant array:{}", std::mem::size_of_val(CONSTANT_ARRAY));
    println!("size of another  array:{}", std::mem::size_of_val(another_array));

    assert_same_type(CONSTANT_ARRAY, another_array);

    let t: &mut i32 = &mut 0;
    foo(t as &i32);

    let b = Baz;
    b.f();
}