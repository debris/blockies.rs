use image::{Rgba, RgbaImage};
use hsl::HSL;
use {hsl_to_rgba, fill_rect};

pub struct Options {
	pub size: u32,
	pub scale: u32,
	pub seed: Vec<u8>,
	pub color: Option<Rgba<u8>>,
	pub background_color: Option<Rgba<u8>>,
	pub spot_color: Option<Rgba<u8>>,
}

pub struct Ethereum {
	randseed: [u32; 4],
}

#[derive(Debug, Clone)]
enum FillType {
	None,
	Color,
	SpotColor,
}

impl Ethereum {
	fn seedrand(&mut self, seed: &[u8]) {
		self.randseed = [0u32; 4];

		for i in 0..seed.len() {
			self.randseed[i % 4] = ((self.randseed[i % 4] << 5) - self.randseed[i % 4]) + seed[i] as u32;
		}
	}

	fn rand(&mut self) -> f64 {
		let t = (self.randseed[0] ^ (self.randseed[0] << 11)) as i32;
		self.randseed[0] = self.randseed[1];
		self.randseed[1] = self.randseed[2];
		self.randseed[2] = self.randseed[3];
		self.randseed[3] = self.randseed[3] ^ (self.randseed[3] >> 19) ^ (t ^ (t >> 8)) as u32;
		
		((self.randseed[3] as i32).abs() as u32) as f64 / ((1u32 << 31) as f64)
	}

	fn create_color(&mut self) -> Rgba<u8> {
		let hsl = HSL {
			h: (self.rand() * 360.0).floor(),
			s: (self.rand() * 60.0 + 40.0) / 100.0,
			l: (self.rand() + self.rand() + self.rand() + self.rand()) * 25.0 / 100.0,
		};
		hsl_to_rgba(hsl)
	}

	fn create_image_data(&mut self, size: u32) -> Vec<FillType> {
		let odd = size % 2 == 1;
		let data_width = size / 2;
	
		(0..size)
			.into_iter()
			.map(|_| {
				let row = (0..data_width)
					.into_iter()
					.map(|_| {
						match (self.rand() * 2.3).floor() {
							0.0 => FillType::None,
							1.0 => FillType::Color,
							_ => FillType::SpotColor,
						}
					})
					.collect::<Vec<_>>();
				let mut cloned_row = row.clone();
				if odd {
					let last = cloned_row.last().cloned().unwrap_or(FillType::None);
					cloned_row.push(last);
				}

				cloned_row.into_iter().chain(row.into_iter().rev()).collect::<Vec<_>>()
			})
			.flat_map(|x| x)
			.collect()
	}

	pub fn create_icon(options: Options) -> RgbaImage {
		let mut builder = Ethereum {
			randseed: [0u32; 4],
		};

		builder.seedrand(&options.seed);
		
		let scale = options.scale;
		let color = options.color.unwrap_or_else(|| builder.create_color());
		let background_color = options.background_color.unwrap_or_else(|| builder.create_color());
		let spot_color = options.spot_color.unwrap_or_else(|| builder.create_color());
		let image_data = builder.create_image_data(options.size);
		let real_size = options.size * scale;
		let mut image = RgbaImage::new(real_size, real_size);
		fill_rect(&mut image, 0, 0, real_size, background_color);

		for (index, fill) in image_data.into_iter().enumerate() {
			let index = index as u32;
			let row = index / options.size;
			let col = index % options.size;

			match fill {
				FillType::None => (),
				FillType::Color => fill_rect(&mut image, col * scale, row * scale, scale, color),
				FillType::SpotColor => fill_rect(&mut image, col * scale, row * scale, scale, spot_color),
			}
		}

		image
	}
}
