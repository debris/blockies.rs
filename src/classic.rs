use hsl::HSL;
use crate::{Rgb, Icon};
use crate::util::{create_image_data, rasterize, hsl_to_rgb};

const fn white() -> Rgb {
	[255, 255, 255]
}

pub struct Options<Seed: AsRef<[u8]>> {
	pub size: usize,
	pub scale: usize,
	pub seed: Seed,
	pub color: Option<Rgb>,
	pub background_color: Option<Rgb>,
}

pub struct Classic {
	randseed: f64,
}

impl Classic {
	fn new(seed: &[u8]) -> Self {
		let mut randseed = 0u64;

		for i in 0..seed.len() / 2 {
			let h = ((seed[i * 2] as u64) << 8) | seed[i * 2 + 1] as u64;
			randseed = randseed ^ h;
		}

		if seed.len() % 2 == 1 {
			randseed = randseed ^ ((seed[seed.len() - 1] as u64) << 8);
		}

		Classic {
			randseed: randseed as f64,
		}
	}

	fn rand(&mut self) -> f64 {
		let n = (self.randseed.sin() + 1.0) / 2.0;
		self.randseed += 1.0;
		let r = n * 10000.0;
		r - r.floor()
	}

	fn create_color(&mut self) -> Rgb {
		let hsl = HSL {
			h: (self.rand() * 360.0).floor(),
			s: (self.rand() * 50.0 + 50.0) / 100.0,
			l: (self.rand() * 60.0 + 20.0) / 100.0,
		};
		hsl_to_rgb(hsl)
	}

	pub(crate) fn create_icon<Seed>(options: Options<Seed>) -> Icon
	where
		Seed: AsRef<[u8]>,
	{
		let mut builder = Classic::new(options.seed.as_ref());

		let scale = options.scale;
		let color = options.color.unwrap_or_else(|| builder.create_color());
		let background_color = options.background_color.unwrap_or_else(|| white());

		let mut palette = Vec::with_capacity(6);

		palette.extend_from_slice(&background_color);
		palette.extend_from_slice(&color);

		let image_data = create_image_data(options.size as usize, || builder.rand() >= 0.5);

		let size = options.size;
		let row_width = size + size % 2;

		let width = row_width * scale;
		let height = size * scale;

		let data = rasterize(&image_data, row_width, size, scale, 1);
		let depth = png::BitDepth::One;

		Icon {
			width,
			height,
			depth,
			palette,
			data,
		}
	}
}
