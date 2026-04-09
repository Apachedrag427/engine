use super::Frame;
use crate::types::{Color, Coordinate2d, CoordinateRect, Rect, Vector2};

impl Frame {
	pub fn fill_rect(&mut self, rect: Rect, color: Color) {
		self.fill_rect_int(rect.into(), color);
	}

	pub fn fill_rect_int(&mut self, rect: CoordinateRect, color: Color) {
		let start = rect.position;
		let mut end = rect.position + rect.dimensions;

		// A scale of 1 means 0 offset.
		end.x -= 1;
		end.y -= 1;

		if start.x == end.x {
			let mut i = start.y * (self.width as isize) + start.x;
			for _y in start.y..=end.y {
				self.set_pixel_i(i as usize, color);

				// Go down a row
				i += self.width as isize;
			}
			return;
		}

		let mut i;
		for y in start.y..=end.y {
			i = y * (self.width as isize) + start.x;
			for _x in start.x..=end.x {
				self.set_pixel_i(i as usize, color);
				i += 1;
			}
		}
	}

	pub fn clear(&mut self, color: Color) {
		self.data.fill(color);
	}

	pub fn callback_fill<T>(&mut self, callback: T)
	where
		T: Fn(usize, usize) -> Color + Send + ToOwned<Owned = T>,
		T: 'static,
	{
		const THREAD_COUNT: usize = 8;
		let width = self.width;
		let height = self.height;
		let length = width * height;
		let mut result: Vec<Color> = Vec::with_capacity(length);
		let mut thread_handles = Vec::with_capacity(THREAD_COUNT);
		for thread_id in 0..THREAD_COUNT {
			let inner_callback = callback.to_owned();
			let thread = std::thread::spawn(move || {
				let start_i = (thread_id as f64 * (length / THREAD_COUNT) as f64).ceil() as usize;
				let mut y = start_i / width;
				let mut x = start_i - (width * y);

				let end_i =
					((thread_id + 1) as f64 * (length / THREAD_COUNT) as f64).ceil() as usize;

				let mut chunk = Vec::with_capacity(end_i - start_i + 1);

				for _i in start_i..end_i {
					chunk.push(inner_callback(x, y));
					x += 1;
					if x == width {
						x = 0;
						y += 1;
					}
				}

				(thread_id, chunk)
			});
			thread_handles.push(thread);
		}

		let mut chunks: Vec<Vec<Color>> = vec![vec![]; THREAD_COUNT];

		for handle in thread_handles {
			let (thread_id, chunk) = handle.join().unwrap();
			chunks[thread_id] = chunk;
		}

		for i in 0..THREAD_COUNT {
			result.append(&mut chunks[i]);
		}

		self.data = result;
	}

	pub fn callback_update<T: Fn(usize, usize, Color) -> Color>(&mut self, callback: T) {
		// Same index rules as callback_fill
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.set_pixel_i(i, callback(x, y, self.data[i]));
				i += 1;
			}
		}
	}

	pub fn draw_frame(&mut self, position: Vector2, frame: Frame) {
		self.draw_frame_int(position.into(), frame);
	}

	pub fn draw_frame_int(&mut self, position: Coordinate2d, frame: Frame) {
		let start = position;
		let mut end = position + frame.get_dimensions();

		// A scale of 1 means 0 offset.
		end.x -= 1;
		end.y -= 1;

		let frame_data = frame.get_raw_data();

		if start.x == end.x {
			let mut i = 0;
			let mut self_i = start.y * (self.width as isize) + start.x;
			for _y in start.y..=end.y {
				self.set_pixel_i(self_i as usize, frame_data[i]);
				i += 1;

				// Go down a row
				self_i += self.width as isize;
			}
			return;
		}

		let mut i = 0;
		let mut self_i;
		for y in start.y..=end.y {
			self_i = y * (self.width as isize) + start.x;
			for _x in start.x..=end.x {
				self.set_pixel_i(self_i as usize, frame_data[i]);
				i += 1;
				self_i += 1;
			}
		}
	}
}
