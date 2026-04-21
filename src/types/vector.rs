use crate::types::Quat;

use super::Lerp;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}

impl Vector2 {
	#[inline]
	pub fn new(x: f64, y: f64) -> Vector2 {
		Vector2 { x, y }
	}
	#[inline]
	pub fn zero() -> Vector2 {
		Vector2::new(0., 0.)
	}
	#[inline]
	pub fn one() -> Vector2 {
		Vector2::new(1., 1.)
	}
	#[inline]
	pub fn x_axis() -> Vector2 {
		Vector2::new(1., 0.)
	}
	#[inline]
	pub fn y_axis() -> Vector2 {
		Vector2::new(0., 1.)
	}

	#[inline]
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	#[inline]
	pub fn abs(&self) -> Vector2 {
		Vector2::new(self.x.abs(), self.y.abs())
	}

	#[inline]
	pub fn normalize(&self) -> Vector2 {
		let magnitude = self.magnitude();
		Vector2 {
			x: self.x / magnitude,
			y: self.y / magnitude,
		}
	}

	#[inline]
	pub fn dot(&self, rhs: Vector2) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y)
	}

	#[inline]
	pub fn cross2d(&self, rhs: Vector2) -> f64 {
		self.x * rhs.y - self.y * rhs.x
	}
}

impl Lerp<Vector2> for Vector2 {
	type Output = Vector2;
	fn lerp(&self, rhs: Vector2, a: f64) -> Self::Output {
		Vector2 {
			x: self.x.lerp(rhs.x, a),
			y: self.y.lerp(rhs.y, a),
		}
	}
}

impl Lerp<Coordinate2d> for Vector2 {
	type Output = Vector2;
	fn lerp(&self, rhs: Coordinate2d, a: f64) -> Self::Output {
		Vector2 {
			x: self.x.lerp(rhs.x, a),
			y: self.y.lerp(rhs.y, a),
		}
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Sub<Vector2> for Vector2 {
	type Output = Vector2;
	fn sub(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl Mul<Vector2> for Vector2 {
	type Output = Vector2;
	fn mul(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x * rhs.x, self.y * rhs.y)
	}
}

impl Mul<f64> for Vector2 {
	type Output = Vector2;
	fn mul(self, rhs: f64) -> Self::Output {
		Vector2::new(self.x * rhs, self.y * rhs)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vector3 {
	#[inline]
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		Vector3 { x, y, z }
	}
	#[inline]
	pub fn zero() -> Vector3 {
		Vector3::new(0., 0., 0.)
	}
	#[inline]
	pub fn one() -> Vector3 {
		Vector3::new(1., 1., 1.)
	}
	#[inline]
	pub fn x_axis() -> Vector3 {
		Vector3::new(1., 0., 0.)
	}
	#[inline]
	pub fn y_axis() -> Vector3 {
		Vector3::new(0., 1., 0.)
	}
	#[inline]
	pub fn z_axis() -> Vector3 {
		Vector3::new(0., 0., 1.)
	}
	#[inline]
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	#[inline]
	pub fn abs(&self) -> Vector3 {
		Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
	}

	#[inline]
	pub fn normalize(&self) -> Vector3 {
		let magnitude = self.magnitude();
		Vector3 {
			x: self.x / magnitude,
			y: self.y / magnitude,
			z: self.z / magnitude,
		}
	}

	#[inline]
	pub fn rotate(&self, rotation: Quat) -> Vector3 {
		let (_, x, y, z) = (rotation * *self * rotation.inverse()).get_components();
		Vector3::new(x, y, z)
	}

	#[inline]
	pub fn translate(&self, translation: Vector3) -> Vector3 {
		*self + translation
	}

	#[inline]
	pub fn dot(&self, rhs: Vector3) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
	}
}

impl Add<Vector3> for Vector3 {
	type Output = Vector3;
	fn add(self, rhs: Vector3) -> Self::Output {
		Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl Sub<Vector3> for Vector3 {
	type Output = Vector3;
	fn sub(self, rhs: Vector3) -> Self::Output {
		Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl Mul<Vector3> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: Vector3) -> Self::Output {
		Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
	}
}

impl Mul<f64> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: f64) -> Self::Output {
		Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl Div<f64> for Vector3 {
	type Output = Vector3;
	fn div(self, rhs: f64) -> Self::Output {
		Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}

impl Neg for Vector3 {
	type Output = Vector3;
	fn neg(self) -> Self::Output {
		Vector3::new(-self.x, -self.y, -self.z)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate2d {
	pub x: isize,
	pub y: isize,
}

impl Coordinate2d {
	#[inline]
	pub fn new(x: isize, y: isize) -> Coordinate2d {
		Coordinate2d { x, y }
	}
	#[inline]
	pub fn zero() -> Coordinate2d {
		Coordinate2d::new(0, 0)
	}
	#[inline]
	pub fn one() -> Coordinate2d {
		Coordinate2d::new(1, 1)
	}
	#[inline]
	pub fn magnitude(&self) -> f64 {
		((self.x * self.x + self.y * self.y) as f64).sqrt()
	}
	#[inline]
	pub fn cross2d(&self, rhs: Coordinate2d) -> isize {
		self.x * rhs.y - self.y * rhs.x
	}
}

impl Lerp<Coordinate2d> for Coordinate2d {
	type Output = Coordinate2d;
	fn lerp(&self, rhs: Coordinate2d, a: f64) -> Self::Output {
		Coordinate2d {
			x: self.x.lerp(rhs.x, a),
			y: self.y.lerp(rhs.y, a),
		}
	}
}

impl Lerp<Vector2> for Coordinate2d {
	type Output = Coordinate2d;
	fn lerp(&self, rhs: Vector2, a: f64) -> Self::Output {
		Coordinate2d {
			x: self.x.lerp(rhs.x, a),
			y: self.y.lerp(rhs.y, a),
		}
	}
}

impl Add<Coordinate2d> for Coordinate2d {
	type Output = Coordinate2d;
	fn add(self, rhs: Coordinate2d) -> Self::Output {
		Coordinate2d::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Sub<Coordinate2d> for Coordinate2d {
	type Output = Coordinate2d;
	fn sub(self, rhs: Coordinate2d) -> Self::Output {
		Coordinate2d::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl Into<Coordinate2d> for Vector2 {
	fn into(self) -> Coordinate2d {
		Coordinate2d {
			x: self.x.round() as isize,
			y: self.y.round() as isize,
		}
	}
}

impl Into<Vector2> for Coordinate2d {
	fn into(self) -> Vector2 {
		Vector2 {
			x: self.x as f64,
			y: self.y as f64,
		}
	}
}
