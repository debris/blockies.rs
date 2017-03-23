extern crate image;
extern crate hsl;

pub mod classic;
pub mod ethereum;

use std::io;
pub use image::ImageError;
use image::{Rgba, RgbaImage, GenericImage, DynamicImage, ImageFormat};
use hsl::HSL;

fn hsl_to_rgba(hsl: HSL) -> Rgba<u8> {
	let (r, g, b) = hsl.to_rgb();
	Rgba::<u8> {
		data: [r, g, b, 1],
	}
}

fn fill_rect(image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
	let mut sub_image = image.sub_image(x, y, size, size);
	for (_, _ , pixel) in sub_image.pixels_mut() {
		*pixel = color;
	}
}

pub enum Blockies {
	Classic(classic::Options),
	Ethereum(ethereum::Options),
}

pub fn create_icon<W>(w: &mut W, blockies: Blockies) -> Result<(), ImageError> where W: io::Write {
	let image = match blockies {
		Blockies::Classic(options) => classic::Classic::create_icon(options),
		Blockies::Ethereum(options) => ethereum::Ethereum::create_icon(options),
	};

	let dy_image = DynamicImage::ImageRgba8(image);
	dy_image.save(w, ImageFormat::PNG)
}
