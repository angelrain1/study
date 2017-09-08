//extern crate h2;

//use h2::server::Server;

trait A {
    fn what_type(&self);
}


impl A for i16 {
    fn what_type(&self) {
        println!("i16");
    }
}

impl A for i32 {
    fn what_type(&self) {
        println!("i32");
    }
}

fn main() {
    let z = 1;
    z.what_type();

    (z as i64).is_positive();
}