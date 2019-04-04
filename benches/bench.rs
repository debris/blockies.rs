#![feature(test)]
extern crate test;

use test::{Bencher, black_box};

#[bench]
fn generate(b: &mut Bencher) {
    use blockies::{create_icon, ethereum};

    let seed = b"\x01\x12\x2d\xf2\xb7\xd1\xc0\xa6\xad\x94\x58\x9d\xa0\x45\xaf\x38\x85\xbe\xdb\xbc";

    b.bytes = 745;

    b.iter(|| {
        let mut memory = Vec::with_capacity(1024);
        let options = ethereum::Options {
            size: 8,
            scale: 16,
            color: None,
            background_color: None,
            spot_color: None,
        };

        black_box(create_icon(&mut memory, seed, options).unwrap());
    });
}
