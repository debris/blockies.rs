#[macro_use]
extern crate clap;
extern crate blockies;

use std::fs::File;
use blockies::{Blockies, ethereum, classic, create_icon};

struct Config {
	blockies: Blockies,
	file: File,
}

fn main() {
	match run() {
		Ok(_) => (),
		Err(err) => {
			println!("{}", err);
		}
	}
}

fn get_config() -> Result<Config, String> {
	let yaml = load_yaml!("cli.yml");
	let matches = clap::App::from_yaml(yaml).get_matches();

	let size: u32 = match matches.value_of("size") {
		Some(size) => size.parse().map_err(|_| "Invalid --size")?,
		None => 10,
	};

	let scale: u32 = match matches.value_of("scale") {
		Some(scale) => scale.parse().map_err(|_| "Invalid --scale")?,
		None => 5,
	};

	let seed: Vec<u8> = match matches.value_of("seed") {
		Some(seed) => seed.into(),
		None => { unreachable!() }
	};

	let blockies = match matches.value_of("mode") {
		Some("ethereum") => Blockies::Ethereum(ethereum::Options {
			size: size,
			scale: scale,
			seed: seed,
			color: None,
			background_color: None,
			spot_color: None,
		}),
		Some("classic") | None => Blockies::Classic(classic::Options {
			size: size,
			scale: scale,
			seed: seed,
			color: None,
			background_color: None,
		}),
		_ => {
			return Err("Invalid --mode".to_owned());
		},
	};

	let output_filename = match matches.value_of("output") {
		Some(filename) => filename,
		None => "blockies.png",
	};

	let file = File::create(output_filename).map_err(|_| format!("Could not create file {}", output_filename))?;

	let config = Config {
		blockies: blockies,
		file: file,
	};
	Ok(config)
}

fn run() -> Result<(), String>  {
	let mut config = get_config()?;
	create_icon(&mut config.file, config.blockies).map_err(|_| "Unexpected error")?;
	Ok(())
}
