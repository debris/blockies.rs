use image::{Rgba, RgbaImage};
use hsl::HSL;
use {hsl_to_rgba, fill_rect};

fn white() -> Rgba<u8> {
	Rgba::<u8> {
		data: [255, 255, 255, 1],
	}
}

pub struct Options {
	pub size: u32,
	pub scale: u32,
	pub seed: Vec<u8>,
	pub color: Option<Rgba<u8>>,
	pub background_color: Option<Rgba<u8>>,
}

pub struct Classic {
	randseed: f64,
}

impl Classic {
	fn seedrand(&mut self, seed: &[u8]) {
		self.randseed = 0.0;

		for i in 0..seed.len() / 2 {
			let h = seed[i * 2] | seed[i * 2 + 1];
			self.randseed = (self.randseed as u8 ^ h) as f64;
		}
	}

	fn rand(&mut self) -> f64 {
		let n = (self.randseed.sin() + 1.0) / 2.0;
		self.randseed += 1.0;
		let r = n * 10000.0;
		r - r.floor()
	}

	fn create_color(&mut self) -> Rgba<u8> {
		let hsl = HSL {
			h: (self.rand() * 360.0).floor(),
			s: (self.rand() * 50.0 + 50.0) / 100.0,
			l: (self.rand() * 60.0 + 20.0) / 100.0,
		};
		hsl_to_rgba(hsl)
	}

	fn create_image_data(&mut self, size: u32) -> Vec<bool> {
		let data_width = size / 2;
	
		(0..size)
			.into_iter()
			.map(|_| {
				let row = (0..data_width)
					.into_iter()
					.map(|_| self.rand() >= 0.5)
					.collect::<Vec<bool>>();
				row.clone().into_iter().chain(row.into_iter().rev()).collect::<Vec<_>>()
			})
			.flat_map(|x| x)
			.collect()
	}

	pub fn create_icon(options: Options) -> RgbaImage {
		let mut builder = Classic {
			randseed: 0.0
		};

		builder.seedrand(&options.seed);

		let scale = options.scale;
		let color = options.color.unwrap_or_else(|| builder.create_color());
		let background_color = options.background_color.unwrap_or_else(|| white());

		let image_data = builder.create_image_data(options.size);
		let real_size = options.size * scale;

		let mut image = RgbaImage::new(real_size, real_size);
		fill_rect(&mut image, 0, 0, real_size, background_color);

		for (index, fill) in image_data.into_iter().enumerate() {
			if fill {
				let index = index as u32;
				let row = index / options.size;
				let col = index % options.size;

				fill_rect(&mut image, col * scale, row * scale, scale, color);
			}
		}

		image
	}
}
