extern crate test;

#[cfg(test)]
mod tests {
    use crate::BfInterpret;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hello_world(b: &mut Bencher) {
        b.iter(|| {
            let bf = BfInterpret::new(
                "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.?".to_string(),
            )
            .unwrap();
            for ret in bf {
                test::black_box(ret);
            }
        });
    }
}
