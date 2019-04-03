use hsl::HSL;
use crate::{Rgb, Icon};
use crate::util::{create_image_data, rasterize, hsl_to_rgb};

pub struct Options<Seed: AsRef<[u8]>> {
	pub size: usize,
	pub scale: usize,
	pub seed: Seed,
	pub color: Option<Rgb>,
	pub background_color: Option<Rgb>,
	pub spot_color: Option<Rgb>,
}

pub struct Ethereum {
	randseed: [i32; 4],
}

#[derive(Debug, Clone, Copy)]
enum FillType {
	Color = 0,
	Background = 1,
	SpotColor = 2,
}

impl Default for FillType {
	fn default() -> Self {
		FillType::Background
	}
}

impl From<FillType> for u8 {
	fn from(fill: FillType) -> u8 {
		fill as u8
	}
}

impl Ethereum {
	fn new(seed: &[u8]) -> Self {
		let mut randseed = [0i32; 4];

		for (i, byte) in seed.iter().enumerate() {
			let (tmp, _) = (randseed[i % 4] << 5).overflowing_sub(randseed[i % 4]);
			randseed[i % 4] = tmp + *byte as i32;
		}

		Ethereum {
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

	fn create_color(&mut self) -> Rgb {
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

	pub(crate) fn create_icon<Seed>(options: Options<Seed>) -> Icon
	where
		Seed: AsRef<[u8]>,
	{
		let mut builder = Ethereum::new(options.seed.as_ref());

		let mut palette = Vec::with_capacity(9);

		palette.extend_from_slice(&options.color.unwrap_or_else(|| builder.create_color()));
		palette.extend_from_slice(&options.background_color.unwrap_or_else(|| builder.create_color()));
		palette.extend_from_slice(&options.spot_color.unwrap_or_else(|| builder.create_color()));

		let image_data = create_image_data(options.size, || builder.create_fill());

		let scale = options.scale;
		let size = options.size;
		let row_width = size + size % 2;

		let width = row_width * scale;
		let height = size * scale;

		let data = rasterize(&image_data, row_width, size, scale, 2);
		let depth = png::BitDepth::Two;

		Icon {
			width,
			height,
			depth,
			palette,
			data,
		}
	}
}
