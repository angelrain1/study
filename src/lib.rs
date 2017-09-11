pub fn do_nothing_slowly() {
    print!(".");
    for _ in 1..10_000_000 {};
}

pub fn do_nothing_fast() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {}

    #[bench]
    fn bench_nothing_slowly(b: &mut Bencher) {
        b.iter(|| do_nothing_slowly());
    }

    #[bench]
    fn bench_nothing_fast(b: &mut Bencher) {
        b.iter(|| do_nothing_fast());
    }
}