#[macro_use]
extern crate clap;

use std::fs::File;
use blockies::{Ethereum, Classic};

fn main() {
	match run() {
		Ok(_) => (),
		Err(err) => {
			eprintln!("{}", err);
		}
	}
}

fn run() -> Result<(), String> {
	let yaml = load_yaml!("cli.yml");
	let matches = clap::App::from_yaml(yaml).get_matches();

	let size: usize = match matches.value_of("size") {
		Some(size) => size.parse().map_err(|_| "Invalid --size")?,
		None => 8,
	};

	let scale: usize = match matches.value_of("scale") {
		Some(scale) => scale.parse().map_err(|_| "Invalid --scale")?,
		None => 16,
	};

	let seed: Vec<u8> = match matches.value_of("seed") {
		Some(seed) => seed.into(),
		None => { unreachable!() }
	};

	let output_filename = match matches.value_of("output") {
		Some(filename) => filename,
		None => "blockies.png",
	};


	match matches.value_of("mode") {
		Some("ethereum") => {
			let mut blockies = Ethereum::default();

			blockies.size = size;
			blockies.scale = scale;

			let file = File::create(output_filename).map_err(|_| format!("Could not create file {}", output_filename))?;

			blockies.create_icon(file, &seed).map_err(|err| format!("Could not render: {:?}", err))
		},
		Some("classic") | None => {
			let mut blockies = Classic::default();

			blockies.size = size;
			blockies.scale = scale;

			let file = File::create(output_filename).map_err(|_| format!("Could not create file {}", output_filename))?;

			blockies.create_icon(file, &seed).map_err(|err| format!("Could not render: {:?}", err))
		},
		_ => {
			Err("Invalid --mode".to_owned())
		},
	}
}
