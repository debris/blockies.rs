use blockies::Ethereum;

fn run_test(expected: &'static [u8], seed: &'static str, size: usize, scale: usize) {
	let mut memory = Vec::new();

	let mut blockies = Ethereum::default();

	blockies.size = size;
	blockies.scale = scale;
	blockies.create_icon(&mut memory, seed.as_bytes()).unwrap();

	assert_eq!(expected, &memory[..]);
}

#[test]
fn test_ethereum() {
	run_test(include_bytes!("./ethereum_hello_world_10_5.png"), "hello world", 10, 5);
	run_test(include_bytes!("./ethereum_0x000528583ba0c881f4d26a1ff50886fc89efc03f_8_16.png"), "0x000528583ba0c881f4d26a1ff50886fc89efc03f", 8, 16);
	run_test(include_bytes!("./ethereum_0x000ba5e704c33c58b5e7949f67344821fa54bd29_8_16.png"), "0x000ba5e704c33c58b5e7949f67344821fa54bd29", 8, 16);
	run_test(include_bytes!("./ethereum_0x01122df2b7d1c0a6ad94589da045af3885bedbbc_8_16.png"), "0x01122df2b7d1c0a6ad94589da045af3885bedbbc", 8, 16);
}
