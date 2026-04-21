use std::ops::Mul;

use crate::types::{Quat, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct CFrame {
	pub position: Vector3,
	pub rotation: Quat,
}

impl CFrame {
	#[inline]
	pub fn new(position: Vector3, rotation: Quat) -> CFrame {
		CFrame { position, rotation }
	}

	#[inline]
	pub fn identity() -> CFrame {
		CFrame {
			position: Vector3::zero(),
			rotation: Quat::identity(),
		}
	}

	#[inline]
	pub fn translate(&self, translation: Vector3) -> CFrame {
		CFrame::new(
			self.position + (translation.rotate(self.rotation)),
			self.rotation,
		)
	}

	#[inline]
	pub fn look_vector(&self) -> Vector3 {
		-Vector3::z_axis().rotate(self.rotation)
	}

	#[inline]
	pub fn right_vector(&self) -> Vector3 {
		Vector3::x_axis().rotate(self.rotation)
	}

	#[inline]
	pub fn up_vector(&self) -> Vector3 {
		Vector3::y_axis().rotate(self.rotation)
	}

	#[inline]
	pub fn rotate(&self, rotation: Quat) -> CFrame {
		CFrame::new(self.position, (self.rotation * rotation).normalize())
	}
}

impl From<Vector3> for CFrame {
	fn from(value: Vector3) -> Self {
		CFrame::new(value, Quat::identity())
	}
}

impl From<Quat> for CFrame {
	fn from(value: Quat) -> Self {
		CFrame {
			position: Vector3::zero(),
			rotation: value,
		}
	}
}

impl Mul<CFrame> for CFrame {
	type Output = CFrame;
	fn mul(self, rhs: CFrame) -> Self::Output {
		CFrame {
			position: self.position + (rhs.position.rotate(self.rotation)),
			rotation: (self.rotation * rhs.rotation).normalize(),
		}
	}
}
