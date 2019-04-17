use std::io;
use hsl::HSL;
use pixelate::{Color, Image, Error};

use crate::util::{create_image_data, hsl_to_rgb};

/// Context struct for creating an Ethereum-style 3-color Blockies.
///
/// The best way to create it is by using the default trait:
///
/// ```rust
/// use blockies::Ethereum;
///
/// let mut gen = Ethereum::default();
/// let mut png = Vec::new();
///
/// gen.scale = 8;
/// gen.create_icon(&mut png, b"0x0000000000000000000000000000000000000000");
///
/// // `png` contains the PNG image of the blockies.
/// assert!(png.len() > 0);
/// ```
pub struct Ethereum {
	/// Size of blockies (number of blocks per row in the image), default: 8
	pub size: usize,
	/// Pixel size (width and height) of a single block in the image, default: 16
	pub scale: usize,
	/// Foreground color of the image, default: None (derived from seed)
	pub color: Option<Color>,
	/// Background color of the image, default: None (derived from seed)
	pub background_color: Option<Color>,
	/// Spot color of the image, default: None (derived form seed)
	pub spot_color: Option<Color>,
}

impl Ethereum {
	/// Write the PNG image of the blockies for a given `seed` into a writer.
	pub fn create_icon<W: io::Write>(&self, writer: W, seed: &[u8]) -> Result<(), Error> {
		let mut seed = Seed::new(seed);

		let palette = vec![
			self.color.unwrap_or_else(|| seed.create_color()),
			self.background_color.unwrap_or_else(|| seed.create_color()),
			self.spot_color.unwrap_or_else(|| seed.create_color()),
		];

		let pixels = create_image_data(self.size, || seed.create_fill());

		Image {
			palette: &palette,
			pixels: &pixels,
			width: self.size + self.size % 2,
			scale: self.scale,
		}.render(writer)
	}
}


impl Default for Ethereum {
	fn default() -> Self {
		Ethereum {
			size: 8,
			scale: 16,
			color: None,
			background_color: None,
			spot_color: None
		}
	}
}

struct Seed {
	randseed: [i32; 4],
}

#[derive(Debug, Clone, Copy)]
enum FillType {
	Color = 0,
	Background = 1,
	SpotColor = 2,
}

impl From<FillType> for u8 {
	fn from(fill: FillType) -> u8 {
		fill as u8
	}
}

impl Seed {
	fn new(seed: &[u8]) -> Self {
		let mut randseed = [0i32; 4];

		for (i, byte) in seed.iter().enumerate() {
			let (tmp, _) = (randseed[i % 4] << 5).overflowing_sub(randseed[i % 4]);
			randseed[i % 4] = tmp + *byte as i32;
		}

		Seed {
			randseed
		}
	}

	fn rand(&mut self) -> f64 {
		let t = self.randseed[0] ^ (self.randseed[0] << 11);
		self.randseed[0] = self.randseed[1];
		self.randseed[1] = self.randseed[2];
		self.randseed[2] = self.randseed[3];
		self.randseed[3] = self.randseed[3] ^ (self.randseed[3] >> 19) ^ (t ^ (t >> 8));

		((self.randseed[3].abs() as f64) / ((1i32 << 31) as f64)).abs()
	}

	fn create_color(&mut self) -> Color {
		let hsl = HSL {
			h: (self.rand() * 360.0).floor(),
			s: (self.rand() * 60.0 + 40.0) / 100.0,
			l: (self.rand() + self.rand() + self.rand() + self.rand()) * 25.0 / 100.0,
		};
		hsl_to_rgb(hsl)
	}

	fn create_fill(&mut self) -> FillType {
		match (self.rand() * 2.3) as u32 {
			0 => FillType::Background,
			1 => FillType::Color,
			_ => FillType::SpotColor,
		}
	}
}
