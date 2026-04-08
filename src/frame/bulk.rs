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

	pub fn callback_fill<T: Fn(usize, usize) -> Color>(&mut self, callback: T) {
		// Use a separate index here to avoid having to recompute it for every pixel
		// The caveat is that I **MUST** loop row by row for the index to line up
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.set_pixel_i(i, callback(x, y));
				i += 1;
			}
		}
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
