use std::io;
pub use png::EncodingError;

pub mod classic;
pub mod ethereum;
pub(crate) mod util;

pub type Rgb = [u8; 3];

pub enum Blockies<Seed: AsRef<[u8]>> {
	Classic(classic::Options<Seed>),
	Ethereum(ethereum::Options<Seed>),
}

struct Icon {
	width: usize,
	height: usize,
	depth: png::BitDepth,
	palette: Vec<u8>,
	data: Vec<u8>,
}

pub fn create_icon<W, Seed>(w: &mut W, blockies: Blockies<Seed>) -> Result<(), EncodingError>
where
	W: io::Write,
	Seed: AsRef<[u8]>,
{
	use png::HasParameters;

	let icon = match blockies {
		Blockies::Classic(options) => classic::Classic::create_icon(options),
		Blockies::Ethereum(options) => ethereum::Ethereum::create_icon(options),
	};

	let mut encoder = png::Encoder::new(w, icon.width as u32, icon.height as u32);

	encoder
		.set(png::ColorType::Indexed)
		.set(icon.depth);

	let mut writer = encoder.write_header()?;

	writer.write_chunk(png::chunk::PLTE, &icon.palette)?;
	writer.write_image_data(&icon.data)
}
