extern crate blockies;
extern crate image;

use blockies::{Blockies, create_icon, ethereum};

#[test]
fn test_ethereum() {
	let expected = image::open("./tests/ethereum_hello_world_10_5.png").expect("expected to find test image");
	let mut expected_memory = Vec::new();
	expected.save(&mut expected_memory, image::ImageFormat::PNG).unwrap();

	let mut memory = Vec::new();
	let options = ethereum::Options {
		size: 10,
		scale: 5,
		seed: "hello world".into(),
		color: None,
		background_color: None,
		spot_color: None,
	};

	create_icon(&mut memory, Blockies::Ethereum(options)).unwrap();
	assert_eq!(expected_memory, memory);
}
