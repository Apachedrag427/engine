use crate::types::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle2d(pub Vector2, pub Vector2, pub Vector2);

#[derive(Debug, Clone, Copy)]
pub struct CoordinateTriangle2d(pub Coordinate2d, pub Coordinate2d, pub Coordinate2d);

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
