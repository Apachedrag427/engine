use std::ops::Mul;

use crate::types::Vector3;

#[derive(Debug, Clone, Copy)]
pub enum RotationOrder {
	XYZ,
	XZY,
	YXZ,
	YZX,
	ZXY,
	ZYX,
}

#[derive(Debug, Clone, Copy)]
pub struct Quat {
	w: f64,
	i: f64,
	j: f64,
	k: f64,
}

impl Quat {
	#[inline]
	pub fn identity() -> Quat {
		Quat {
			w: 1.,
			i: 0.,
			j: 0.,
			k: 0.,
		}
	}

	pub fn from_rotation_around_axis(angle: f64, axis: Vector3) -> Quat {
		let axis = axis.normalize();
		// divide angle by 2 because the inverse quaternion will provide the
		// other half of the rotation
		let sine = f64::sin(angle / 2.);
		Quat {
			w: f64::cos(angle / 2.),
			i: sine * axis.x,
			j: sine * axis.y,
			k: sine * axis.z,
		}
	}

	pub fn from_euler_angles(
		x_angle: f64,
		y_angle: f64,
		z_angle: f64,
		order: RotationOrder,
	) -> Quat {
		let x = Quat::from_rotation_around_axis(x_angle, Vector3::x_axis());
		let y = Quat::from_rotation_around_axis(y_angle, Vector3::y_axis());
		let z = Quat::from_rotation_around_axis(z_angle, Vector3::z_axis());

		match order {
			RotationOrder::XYZ => x * y * z,
			RotationOrder::XZY => x * z * y,
			RotationOrder::YXZ => y * x * z,
			RotationOrder::YZX => y * z * x,
			RotationOrder::ZXY => z * x * y,
			RotationOrder::ZYX => z * y * x,
		}
	}

	#[inline]
	pub fn get_components(&self) -> (f64, f64, f64, f64) {
		(self.w, self.i, self.j, self.k)
	}

	#[inline]
	pub fn get_angle(&self) -> f64 {
		// acos(self.w) * 2. works, but is numerically unstable near self.w = +-1
		f64::atan2(
			f64::sqrt(self.i * self.i + self.j * self.j + self.k * self.k),
			self.w,
		) * 2.
	}

	#[inline]
	pub fn get_axis(&self) -> Vector3 {
		let sine = f64::sin(self.get_angle());
		Vector3::new(self.i / sine, self.j / sine, self.k / sine)
	}

	#[inline]
	pub fn inverse(&self) -> Quat {
		Quat {
			w: self.w,
			i: -self.i,
			j: -self.j,
			k: -self.k,
		}
	}

	#[inline]
	pub fn normalize(&self) -> Quat {
		let mag = f64::sqrt(self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k);
		Quat {
			w: self.w / mag,
			i: self.i / mag,
			j: self.j / mag,
			k: self.k / mag,
		}
	}
}

impl Mul<Quat> for Quat {
	type Output = Quat;
	fn mul(self, rhs: Quat) -> Self::Output {
		Quat {
			w: self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
			i: self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
			j: self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
			k: self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w,
		}
	}
}

impl Mul<Quat> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: Quat) -> Self::Output {
		Vector3::new(
			self.x * rhs.w + self.y * rhs.k - self.z * rhs.j,
			self.x * rhs.k + self.y * rhs.w + self.z * rhs.i,
			self.x * rhs.j - self.y * rhs.i + self.z * rhs.w,
		)
	}
}

impl Mul<Vector3> for Quat {
	type Output = Quat;
	fn mul(self, rhs: Vector3) -> Self::Output {
		Quat {
			w: self.i * rhs.x - self.j * rhs.y - self.k * rhs.z,
			i: self.w * rhs.x + self.j * rhs.z - self.k * rhs.y,
			j: self.w * rhs.y - self.i * rhs.z + self.k * rhs.x,
			k: self.w * rhs.z + self.i * rhs.y - self.j * rhs.x,
		}
	}
}
