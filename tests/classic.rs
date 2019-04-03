use blockies::{Blockies, create_icon, classic};

#[test]
fn test_classic() {
	let expected = include_bytes!("./classic_hello_world_10_5.png");

	let mut memory = Vec::new();
	let options = classic::Options {
		size: 10,
		scale: 5,
		seed: "hello world",
		color: None,
		background_color: None,
	};

	create_icon(&mut memory, Blockies::Classic(options)).unwrap();

	assert_eq!(&expected[..], &memory[..]);
}
