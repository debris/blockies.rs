use blockies::Classic;

#[test]
fn test_classic() {
	let expected = include_bytes!("./classic_hello_world_10_5.png");

	let mut memory = Vec::new();
	let mut blockies = Classic::default();

	blockies.size = 10;
	blockies.scale = 5;
	blockies.create_icon(&mut memory, b"hello world").unwrap();

	assert_eq!(&expected[..], &memory[..]);
}
