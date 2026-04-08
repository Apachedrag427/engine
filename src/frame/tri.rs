use super::*;
use crate::types::{Coordinate2d, CoordinateTriangle2d, Triangle2d};

impl Frame {
	pub fn draw_wireframe_tri(&mut self, tri: Triangle2d, color: Color) {
		self.draw_wireframe_tri_int(tri.into(), color);
	}

	pub fn draw_wireframe_tri_int(&mut self, tri: CoordinateTriangle2d, color: Color) {
		self.draw_line_int(tri.0, tri.1, color);
		self.draw_line_int(tri.1, tri.2, color);
		self.draw_line_int(tri.2, tri.0, color);
	}

	// Explanation of triangle drawing algorithms: https://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
	pub fn draw_tri(&mut self, tri: Triangle2d, color: Color) {
		self.draw_tri_int(tri.into(), color);
	}

	pub fn draw_tri_int(&mut self, tri: CoordinateTriangle2d, color: Color) {
		let mut v1 = tri.0;
		let mut v2 = tri.1;
		let mut v3 = tri.2;

		// sort the verts so v1 is the topmost
		if v1.y < v2.y {
			(v1, v2) = (v2, v1);
		}
		if v2.y < v3.y {
			(v2, v3) = (v3, v2);
		}
		if v1.y < v2.y {
			(v1, v2) = (v2, v1);
		}

		let tri = CoordinateTriangle2d(v1, v2, v3);

		if v2.y == v3.y {
			self.draw_flat_bottom_tri(tri, color);
		} else if v1.y == v2.y {
			self.draw_flat_top_tri(tri, color);
		} else {
			let v4 = Coordinate2d::new(
				(v1.x as f64 + ((v2.y - v1.y) as f64 / (v3.y - v1.y) as f64) * (v3.x - v1.x) as f64)
					.round() as isize,
				v2.y,
			);
			self.draw_flat_bottom_tri(CoordinateTriangle2d(v1, v2, v4), color);
			self.draw_flat_top_tri(CoordinateTriangle2d(v2, v4, v3), color);
		}
	}

	fn draw_flat_bottom_tri(&mut self, tri: CoordinateTriangle2d, color: Color) {
		let v1 = tri.0;
		let v2 = tri.1;
		let v3 = tri.2;

		let inv_slope1 = (v2.x - v1.x) as f64 / (v2.y - v1.y) as f64;
		let inv_slope2 = (v3.x - v1.x) as f64 / (v3.y - v1.y) as f64;

		let mut cur_x1 = v1.x as f64;
		let mut cur_x2 = v1.x as f64;

		for scanline_y in v1.y..=v2.y {
			// TODO: Use a more optimized method for drawing a flat line
			self.draw_line_int(
				Coordinate2d::new(cur_x1.round() as isize, scanline_y),
				Coordinate2d::new(cur_x2.round() as isize, scanline_y),
				color,
			);

			cur_x1 += inv_slope1;
			cur_x2 += inv_slope2;
		}
	}

	fn draw_flat_top_tri(&mut self, tri: CoordinateTriangle2d, color: Color) {
		let v1 = tri.0;
		let v2 = tri.1;
		let v3 = tri.2;

		let inv_slope1 = (v3.x - v1.x) as f64 / (v3.y - v1.y) as f64;
		let inv_slope2 = (v3.x - v2.x) as f64 / (v3.y - v2.y) as f64;

		let mut cur_x1 = v3.x as f64;
		let mut cur_x2 = v3.x as f64;

		for scanline_y in v3.y..v1.y {
			// TODO: Use a more optimized method for drawing a flat line
			self.draw_line_int(
				Coordinate2d::new(cur_x1.round() as isize, scanline_y),
				Coordinate2d::new(cur_x2.round() as isize, scanline_y),
				color,
			);

			cur_x1 -= inv_slope1;
			cur_x2 -= inv_slope2;
		}
	}
}
