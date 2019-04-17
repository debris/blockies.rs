#![feature(test)]
extern crate test;

use test::{Bencher, black_box};
use blockies::Ethereum;

#[bench]
fn generate(b: &mut Bencher) {
    let seed = b"\x01\x12\x2d\xf2\xb7\xd1\xc0\xa6\xad\x94\x58\x9d\xa0\x45\xaf\x38\x85\xbe\xdb\xbc";
    let blockies = Ethereum::default();

    b.bytes = 745;

    b.iter(|| {
        let mut memory = Vec::with_capacity(1024);

        black_box(blockies.create_icon(&mut memory, seed).unwrap());
    });
}
