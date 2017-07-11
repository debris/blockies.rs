extern crate blockies;
extern crate image;

use blockies::{Blockies, create_icon, ethereum};

fn run_test(path: &'static str, seed: &'static str, size: u32, scale: u32) {
	let expected = image::open(path).expect("expected to find test image");
	let mut expected_memory = Vec::new();
	expected.save(&mut expected_memory, image::ImageFormat::PNG).unwrap();

	let mut memory = Vec::new();
	let options = ethereum::Options {
		size: size,
		scale: scale,
		seed: seed.into(),
		color: None,
		background_color: None,
		spot_color: None,
	};

	create_icon(&mut memory, Blockies::Ethereum(options)).unwrap();
	assert_eq!(expected_memory, memory);
}

#[test]
fn test_ethereum() {
	run_test("./tests/ethereum_hello_world_10_5.png", "hello world", 10, 5);
	run_test("./tests/ethereum_0x000528583ba0c881f4d26a1ff50886fc89efc03f_8_16.png", "0x000528583ba0c881f4d26a1ff50886fc89efc03f", 8, 16);
	run_test("./tests/ethereum_0x000ba5e704c33c58b5e7949f67344821fa54bd29_8_16.png", "0x000ba5e704c33c58b5e7949f67344821fa54bd29", 8, 16);
	run_test("./tests/ethereum_0x01122df2b7d1c0a6ad94589da045af3885bedbbc_8_16.png", "0x01122df2b7d1c0a6ad94589da045af3885bedbbc", 8, 16);
}
