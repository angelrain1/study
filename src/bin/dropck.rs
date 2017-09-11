use std::cell::Cell;

#[macro_use]
extern crate lazy_static;

struct Concrete<'a>(u32, Cell<Option<&'a Concrete<'a>>>);

struct Foo2<T> { data: Vec<T> }

// This is the new `impl Drop`
//impl<T> Drop for Foo2<T> {
//    fn drop(&mut self) {}
//}

static OK: &'static str = "OK";
static TEST: &'static str = "test";

struct A(u8);

impl A {
    pub fn test(_: &str) -> &'static str { OK }
}

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, fn(&str) -> &'static str> = {
        let mut m: HashMap<&'static str, fn(&str) -> &'static str> = HashMap::new();
        m.insert(TEST, A::test);
        m
    };
}

trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }

impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

fn main() {
    let map: &HashMap<&'static str, fn(&str) -> &'static str> = &HASHMAP;
    map.get("123").map(|f| { println!("{:p}", f); });

    let mut foo1 = Foo2 { data: Vec::new() };
    foo1.data.push(Concrete(0, Cell::new(None)));
    foo1.data.push(Concrete(0, Cell::new(None)));

    {
        foo1.data[0].1.set(Some(&foo1.data[1]));
        foo1.data[1].1.set(Some(&foo1.data[0]));
    }

    println!("{}", std::mem::size_of::<&Foo>());
    println!("{}", std::mem::size_of::<*mut ()>() * 2);
}