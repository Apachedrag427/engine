use crate::types::{Bounds, CoordinateBounds, CoordinateRect, Rect, vector::*};

#[derive(PartialEq, Clone)]
pub enum Winding {
	CW,
	CCW,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle2d(pub Vector2, pub Vector2, pub Vector2);

impl Triangle2d {
	pub fn get_bounds(&self) -> Bounds {
		Bounds {
			min: Vector2::new(
				f64::min(f64::min(self.0.x, self.1.x), self.2.x),
				f64::min(f64::min(self.0.y, self.1.y), self.2.y),
			),
			max: Vector2::new(
				f64::max(f64::max(self.0.x, self.1.x), self.2.x),
				f64::max(f64::max(self.0.y, self.1.y), self.2.y),
			),
		}
	}

	pub fn get_bounds_rect(&self) -> Rect {
		let position = Vector2::new(
			f64::min(f64::min(self.0.x, self.1.x), self.2.x),
			f64::min(f64::min(self.0.y, self.1.y), self.2.y),
		);
		Rect {
			position,
			dimensions: Vector2::new(
				f64::max(f64::max(self.0.x, self.1.x), self.2.x) - position.x,
				f64::max(f64::max(self.0.y, self.1.y), self.2.y) - position.y,
			),
		}
	}

	pub fn get_winding(&self) -> Winding {
		if (self.1 - self.0).cross2d(self.2 - self.0) >= 0. {
			Winding::CW
		} else {
			Winding::CCW
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateTriangle2d(pub Coordinate2d, pub Coordinate2d, pub Coordinate2d);

impl CoordinateTriangle2d {
	pub fn get_bounds(&self) -> CoordinateBounds {
		CoordinateBounds {
			min: Coordinate2d::new(
				isize::min(isize::min(self.0.x, self.1.x), self.2.x),
				isize::min(isize::min(self.0.y, self.1.y), self.2.y),
			),
			max: Coordinate2d::new(
				isize::max(isize::max(self.0.x, self.1.x), self.2.x),
				isize::max(isize::max(self.0.y, self.1.y), self.2.y),
			),
		}
	}

	pub fn get_bounds_rect(&self) -> CoordinateRect {
		let position = Coordinate2d::new(
			isize::min(isize::min(self.0.x, self.1.x), self.2.x),
			isize::min(isize::min(self.0.y, self.1.y), self.2.y),
		);
		CoordinateRect {
			position,
			dimensions: Coordinate2d::new(
				isize::max(isize::max(self.0.x, self.1.x), self.2.x) - position.x,
				isize::max(isize::max(self.0.y, self.1.y), self.2.y) - position.y,
			),
		}
	}

	pub fn get_winding(&self) -> Winding {
		if (self.1 - self.0).cross2d(self.2 - self.0) >= 0 {
			Winding::CW
		} else {
			Winding::CCW
		}
	}
}

impl Into<CoordinateTriangle2d> for Triangle2d {
	fn into(self) -> CoordinateTriangle2d {
		CoordinateTriangle2d(self.0.into(), self.1.into(), self.2.into())
	}
}

impl Into<Triangle2d> for CoordinateTriangle2d {
	fn into(self) -> Triangle2d {
		Triangle2d(self.0.into(), self.1.into(), self.2.into())
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle3d(pub Vector3, pub Vector3, pub Vector3);

impl Triangle3d {
	pub fn get_centroid(&self) -> Vector3 {
		(self.0 + self.1 + self.2) / 3.
	}
}
