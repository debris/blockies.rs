extern crate blockies;
extern crate image;

use blockies::{Blockies, create_icon, classic};

#[test]
fn test_classic() {
	let expected = image::open("./tests/classic_hello_world_10_5.png").expect("expected to find test image");
	let mut expected_memory = Vec::new();
	expected.save(&mut expected_memory, image::ImageFormat::PNG).unwrap();

	let mut memory = Vec::new();
	let options = classic::Options {
		size: 10,
		scale: 5,
		seed: "hello world".into(),
		color: None,
		background_color: None,
	};

	create_icon(&mut memory, Blockies::Classic(options)).unwrap();
	assert_eq!(expected_memory, memory);
}
