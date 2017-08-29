fn main() {
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