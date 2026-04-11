use crate::types::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
	pub min: Vector2,
	pub max: Vector2,
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateBounds {
	pub min: Coordinate2d,
	pub max: Coordinate2d,
}

impl Into<CoordinateBounds> for Bounds {
	fn into(self) -> CoordinateBounds {
		CoordinateBounds {
			min: self.min.into(),
			max: self.max.into(),
		}
	}
}

impl Into<Bounds> for CoordinateBounds {
	fn into(self) -> Bounds {
		Bounds {
			min: self.min.into(),
			max: self.max.into(),
		}
	}
}
