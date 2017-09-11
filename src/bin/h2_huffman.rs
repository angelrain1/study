struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context<'a>(context: &'a Context) -> Result<(), &'a str> {
    Parser { context }.parse()
}

struct Ref<'a, T: 'a>(&'a T);

trait Foo {
    fn p(&self);
}

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> {
    fn p(&self) {
        println!("{}", self.x);
    }
}


struct Count(u32);

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator<u32> for Count {
    fn next(&mut self) -> Option<u32> {
        let next = self.0;
        self.0 = next + 1;
        Some(next)
    }
}

impl Iterator<()> for Count {
    fn next(&mut self) -> Option<()> {
        Some(())
    }
}

fn a<T: std::fmt::Debug, I: Iterator<T>>(mut i: I) {
    if let Some(t) = i.next() {
        println!("{:?}", t);
    }
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn loop_forever() -> ! {
    println!("forever ");

    loop {
        println!("and ever ");

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn b<'r, 'a: 'r, T: Sized>(s: &'a T) -> &'r T {
    s
}

static HUFFMAN_CODE_TABLE: &'static [(u32, u8)] = &[
    (0x0000_1ff8, 13),
    (0x007f_ffd8, 23),
    (0x0fff_ffe2, 28),
    (0x0fff_ffe3, 28),
    (0x0fff_ffe4, 28),
    (0x0fff_ffe5, 28),
    (0x0fff_ffe6, 28),
    (0x0fff_ffe7, 28),
    (0x0fff_ffe8, 28),
    (0x00ff_ffea, 24),
    (0x3fff_fffc, 30),
    (0x0fff_ffe9, 28),
    (0x0fff_ffea, 28),
    (0x3fff_fffd, 30),
    (0x0fff_ffeb, 28),
    (0x0fff_ffec, 28),
    (0x0fff_ffed, 28),
    (0x0fff_ffee, 28),
    (0x0fff_ffef, 28),
    (0x0fff_fff0, 28),
    (0x0fff_fff1, 28),
    (0x0fff_fff2, 28),
    (0x3fff_fffe, 30),
    (0x0fff_fff3, 28),
    (0x0fff_fff4, 28),
    (0x0fff_fff5, 28),
    (0x0fff_fff6, 28),
    (0x0fff_fff7, 28),
    (0x0fff_fff8, 28),
    (0x0fff_fff9, 28),
    (0x0fff_fffa, 28),
    (0x0fff_fffb, 28),
    (0x0000_0014, 6),
    (0x0000_03f8, 10),
    (0x0000_03f9, 10),
    (0x0000_0ffa, 12),
    (0x0000_1ff9, 13),
    (0x0000_0015, 6),
    (0x0000_00f8, 8),
    (0x0000_07fa, 11),
    (0x0000_03fa, 10),
    (0x0000_03fb, 10),
    (0x0000_00f9, 8),
    (0x0000_07fb, 11),
    (0x0000_00fa, 8),
    (0x0000_0016, 6),
    (0x0000_0017, 6),
    (0x0000_0018, 6),
    (0x0000_0000, 5),
    (0x0000_0001, 5),
    (0x0000_0002, 5),
    (0x0000_0019, 6),
    (0x0000_001a, 6),
    (0x0000_001b, 6),
    (0x0000_001c, 6),
    (0x0000_001d, 6),
    (0x0000_001e, 6),
    (0x0000_001f, 6),
    (0x0000_005c, 7),
    (0x0000_00fb, 8),
    (0x0000_7ffc, 15),
    (0x0000_0020, 6),
    (0x0000_0ffb, 12),
    (0x0000_03fc, 10),
    (0x0000_1ffa, 13),
    (0x0000_0021, 6),
    (0x0000_005d, 7),
    (0x0000_005e, 7),
    (0x0000_005f, 7),
    (0x0000_0060, 7),
    (0x0000_0061, 7),
    (0x0000_0062, 7),
    (0x0000_0063, 7),
    (0x0000_0064, 7),
    (0x0000_0065, 7),
    (0x0000_0066, 7),
    (0x0000_0067, 7),
    (0x0000_0068, 7),
    (0x0000_0069, 7),
    (0x0000_006a, 7),
    (0x0000_006b, 7),
    (0x0000_006c, 7),
    (0x0000_006d, 7),
    (0x0000_006e, 7),
    (0x0000_006f, 7),
    (0x0000_0070, 7),
    (0x0000_0071, 7),
    (0x0000_0072, 7),
    (0x0000_00fc, 8),
    (0x0000_0073, 7),
    (0x0000_00fd, 8),
    (0x0000_1ffb, 13),
    (0x0007_fff0, 19),
    (0x0000_1ffc, 13),
    (0x0000_3ffc, 14),
    (0x0000_0022, 6),
    (0x0000_7ffd, 15),
    (0x0000_0003, 5),
    (0x0000_0023, 6),
    (0x0000_0004, 5),
    (0x0000_0024, 6),
    (0x0000_0005, 5),
    (0x0000_0025, 6),
    (0x0000_0026, 6),
    (0x0000_0027, 6),
    (0x0000_0006, 5),
    (0x0000_0074, 7),
    (0x0000_0075, 7),
    (0x0000_0028, 6),
    (0x0000_0029, 6),
    (0x0000_002a, 6),
    (0x0000_0007, 5),
    (0x0000_002b, 6),
    (0x0000_0076, 7),
    (0x0000_002c, 6),
    (0x0000_0008, 5),
    (0x0000_0009, 5),
    (0x0000_002d, 6),
    (0x0000_0077, 7),
    (0x0000_0078, 7),
    (0x0000_0079, 7),
    (0x0000_007a, 7),
    (0x0000_007b, 7),
    (0x0000_7ffe, 15),
    (0x0000_07fc, 11),
    (0x0000_3ffd, 14),
    (0x0000_1ffd, 13),
    (0x0fff_fffc, 28),
    (0x000f_ffe6, 20),
    (0x003f_ffd2, 22),
    (0x000f_ffe7, 20),
    (0x000f_ffe8, 20),
    (0x003f_ffd3, 22),
    (0x003f_ffd4, 22),
    (0x003f_ffd5, 22),
    (0x007f_ffd9, 23),
    (0x003f_ffd6, 22),
    (0x007f_ffda, 23),
    (0x007f_ffdb, 23),
    (0x007f_ffdc, 23),
    (0x007f_ffdd, 23),
    (0x007f_ffde, 23),
    (0x00ff_ffeb, 24),
    (0x007f_ffdf, 23),
    (0x00ff_ffec, 24),
    (0x00ff_ffed, 24),
    (0x003f_ffd7, 22),
    (0x007f_ffe0, 23),
    (0x00ff_ffee, 24),
    (0x007f_ffe1, 23),
    (0x007f_ffe2, 23),
    (0x007f_ffe3, 23),
    (0x007f_ffe4, 23),
    (0x001f_ffdc, 21),
    (0x003f_ffd8, 22),
    (0x007f_ffe5, 23),
    (0x003f_ffd9, 22),
    (0x007f_ffe6, 23),
    (0x007f_ffe7, 23),
    (0x00ff_ffef, 24),
    (0x003f_ffda, 22),
    (0x001f_ffdd, 21),
    (0x000f_ffe9, 20),
    (0x003f_ffdb, 22),
    (0x003f_ffdc, 22),
    (0x007f_ffe8, 23),
    (0x007f_ffe9, 23),
    (0x001f_ffde, 21),
    (0x007f_ffea, 23),
    (0x003f_ffdd, 22),
    (0x003f_ffde, 22),
    (0x00ff_fff0, 24),
    (0x001f_ffdf, 21),
    (0x003f_ffdf, 22),
    (0x007f_ffeb, 23),
    (0x007f_ffec, 23),
    (0x001f_ffe0, 21),
    (0x001f_ffe1, 21),
    (0x003f_ffe0, 22),
    (0x001f_ffe2, 21),
    (0x007f_ffed, 23),
    (0x003f_ffe1, 22),
    (0x007f_ffee, 23),
    (0x007f_ffef, 23),
    (0x000f_ffea, 20),
    (0x003f_ffe2, 22),
    (0x003f_ffe3, 22),
    (0x003f_ffe4, 22),
    (0x007f_fff0, 23),
    (0x003f_ffe5, 22),
    (0x003f_ffe6, 22),
    (0x007f_fff1, 23),
    (0x03ff_ffe0, 26),
    (0x03ff_ffe1, 26),
    (0x000f_ffeb, 20),
    (0x0007_fff1, 19),
    (0x003f_ffe7, 22),
    (0x007f_fff2, 23),
    (0x003f_ffe8, 22),
    (0x01ff_ffec, 25),
    (0x03ff_ffe2, 26),
    (0x03ff_ffe3, 26),
    (0x03ff_ffe4, 26),
    (0x07ff_ffde, 27),
    (0x07ff_ffdf, 27),
    (0x03ff_ffe5, 26),
    (0x00ff_fff1, 24),
    (0x01ff_ffed, 25),
    (0x0007_fff2, 19),
    (0x001f_ffe3, 21),
    (0x03ff_ffe6, 26),
    (0x07ff_ffe0, 27),
    (0x07ff_ffe1, 27),
    (0x03ff_ffe7, 26),
    (0x07ff_ffe2, 27),
    (0x00ff_fff2, 24),
    (0x001f_ffe4, 21),
    (0x001f_ffe5, 21),
    (0x03ff_ffe8, 26),
    (0x03ff_ffe9, 26),
    (0x0fff_fffd, 28),
    (0x07ff_ffe3, 27),
    (0x07ff_ffe4, 27),
    (0x07ff_ffe5, 27),
    (0x000f_ffec, 20),
    (0x00ff_fff3, 24),
    (0x000f_ffed, 20),
    (0x001f_ffe6, 21),
    (0x003f_ffe9, 22),
    (0x001f_ffe7, 21),
    (0x001f_ffe8, 21),
    (0x007f_fff3, 23),
    (0x003f_ffea, 22),
    (0x003f_ffeb, 22),
    (0x01ff_ffee, 25),
    (0x01ff_ffef, 25),
    (0x00ff_fff4, 24),
    (0x00ff_fff5, 24),
    (0x03ff_ffea, 26),
    (0x007f_fff4, 23),
    (0x03ff_ffeb, 26),
    (0x07ff_ffe6, 27),
    (0x03ff_ffec, 26),
    (0x03ff_ffed, 26),
    (0x07ff_ffe7, 27),
    (0x07ff_ffe8, 27),
    (0x07ff_ffe9, 27),
    (0x07ff_ffea, 27),
    (0x07ff_ffeb, 27),
    (0x0fff_fffe, 28),
    (0x07ff_ffec, 27),
    (0x07ff_ffed, 27),
    (0x07ff_ffee, 27),
    (0x07ff_ffef, 27),
    (0x07ff_fff0, 27),
    (0x03ff_ffee, 26),
    (0x3fff_ffff, 30),
];

enum HuffmanCode {
    A(u8),
    B(u8, u8),
}

enum HuffmanTree<'a> {
    Leaf(HuffmanCode),
    Node([Option<&'a HuffmanTree<'a>>; 256]),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HuffmanNode {
    A(u8, (u32, u8)),
    B(u8, u8, (u32, u8)),
    C(u8, Vec<HuffmanNode>),
    D(u8, u8, Vec<HuffmanNode>),
    E,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HuffmanNode2 {
    A(u8, u32, u8),
    B(u8, u8, u32, u8),
    C(u8, u8, u8, u32, u8),
    D(u8, u8, u8, u8, u32, u8),
}

fn level0() -> HuffmanTree<'static> {
    HuffmanTree::Node([None; 256])
}

use std::collections::HashMap;

fn main() {
    let mut vec: Vec<HuffmanNode2> = HUFFMAN_CODE_TABLE
        .iter()
        .map(|&(a, b)| {
            match b {
                i if b > 24 => {
                    let u0 = ((a >> (i - 8)) & 0xFF) as u8;
                    let u1 = ((a >> (i - 16)) & 0xFF) as u8;
                    let u2 = ((a >> (i - 24)) & 0xFF) as u8;
                    let u3 = (a & (0xFF >> (32 - i))) as u8;
                    HuffmanNode2::D(u0, u1, u2, u3, a, b)
                }
                i if b > 16 => {
                    let u0 = ((a >> (i - 8)) & 0xFF) as u8;
                    let u1 = ((a >> (i - 16)) & 0xFF) as u8;
                    let u2 = (a & (0xFF >> (24 - i))) as u8;
                    HuffmanNode2::C(u0, u1, u2, a, b)
                }
                i if b > 8 => {
                    let u0 = ((a >> (i - 8)) & 0xFF) as u8;
                    let u1 = (a & (0xFF >> (16 - i))) as u8;
                    HuffmanNode2::B(u0, u1, a, b)
                }
                _ => {
                    HuffmanNode2::A(a as u8, a, b)
                }
            }
        })
        .collect();

    vec.sort();

    for i in vec {
        println!("vec:{:?}", i);
    }

    for &(a, b) in HUFFMAN_CODE_TABLE {
        let mut i = b;
        let mut aaa = vec![];
        while i > 0 {
            if i > 8 {
                i -= 8;
                let u = (a >> i) as u8;
                aaa.push(HuffmanNode::C(u, vec![]));
                //                map.entry(u).or_insert(HuffmanNode::C(u, vec![]));
            } else {
                if i == 8 {
                    let u = a as u8;
                    aaa.push(HuffmanNode::A(u, (a, b)));
                } else {
                    let u = (a as u8) << (8 - i);
                    aaa.push(HuffmanNode::B(u, u + ((1u8 << (8 - i)) - 1), (a, b)));
                }
                i = 0;
            }
        }
    }


    let mut map = HashMap::new();
    for &(a, b) in HUFFMAN_CODE_TABLE {
        map.entry(b)
            .or_insert_with(|| vec![])
            .push(a);
    }

    let mut vec: Vec<(u8, Vec<u32>)> = map.drain().collect();

    vec.sort();

    for (a, b) in vec {
        println!("len:{},size:{}", a, b.len());
    }


    let ssss = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

    println!("ssss.len:{}", ssss.len());
    println!("ssss.as_bytes().len:{}", ssss.as_bytes().len());

    for a in ssss.as_bytes() {
        print!("{:02x}", a);
    }
    println!();
    println!("{}", "505249202a20485454502f322e300d0a0d0a534d0d0a0d0a");


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let bbb: &str = "aabc";
    let str = String::from(bbb);
    let ctx = Context(&str);
    if let Err(e) = parse_context(&ctx) {
        println!("{}", e);
    }

    let mut c = Count(0);

    if let Some(t) = <Count as Iterator<u32>>::next(&mut c) {
        println!("{}", t);
    }

    if let Some(t) = <Count as Iterator<()>>::next(&mut c) {
        println!("{:?}", t);
    }

    a::<(), Count>(c);


    let mut num = 5;

    {
        let obj = Box::new(Bar { x: &num }) as Box<Foo>;
        println!("{:p}", obj);
        obj.p();
    }

    num = 10;

    println!("{}", num);

    let a = Ref(&bbb);
    println!("{:p}", &a);

    b(&[0u32; 20]);
    b(&str);

    loop_forever();
}