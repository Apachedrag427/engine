use crate::types::{CFrame, Lerp, Triangle3d, Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub enum CameraProjection {
	Orthographic,
	Perspective,
}

#[derive(Debug, Clone)]
pub struct Camera {
	pub cframe: CFrame,
	pub scale: f64,
	pub fov: f64,
	pub near_dist: f64,
	pub far_dist: f64,
	// easier than aspect ratio alone
	pub screen_dimensions: Vector2,
	pub projection: CameraProjection,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			cframe: CFrame::identity(),
			fov: 70.,
			scale: 1.,
			near_dist: 0.01,
			far_dist: 10.,
			screen_dimensions: Vector2::new(4., 3.),
			projection: CameraProjection::Perspective,
		}
	}

	pub fn transform_point(&self, point: Vector3) -> Vector3 {
		(point - self.cframe.position).rotate(self.cframe.rotation.inverse())
	}

	pub fn transform_triangle(&self, tri: Triangle3d) -> Triangle3d {
		Triangle3d(
			self.transform_point(tri.0),
			self.transform_point(tri.1),
			self.transform_point(tri.2),
		)
	}

	// Returns a point in the range (0, 0, 0) to (1, 1, 1)
	// positive x is right, positive y is down, and positive z is forwards
	pub fn project_point(&self, point: Vector3) -> Vector3 {
		let relative_point = (point - self.cframe.position).rotate(self.cframe.rotation.inverse());
		let plane_corner = self.screen_dimensions.normalize() * self.scale;
		match self.projection {
			CameraProjection::Orthographic => Vector3::new(
				(relative_point.x / plane_corner.x) * 0.5 + 0.5,
				(-(relative_point.y / plane_corner.y)) * 0.5 + 0.5,
				(relative_point.z + self.near_dist) / (-self.far_dist + self.near_dist),
			),
			CameraProjection::Perspective => {
				let n = self.near_dist;
				let f = self.far_dist;
				let r = plane_corner.y / plane_corner.x;
				let x = relative_point.x;
				let y = -relative_point.y;
				let z = -relative_point.z;

				Vector3::new(
					x * r / f64::tan(self.fov.to_radians() / 2.) / z * 0.5 + 0.5,
					y / f64::tan(self.fov.to_radians() / 2.) / z * 0.5 + 0.5,
					(z * (f / (f - n)) - (f * n / (f - n))) / z * 0.5 + 0.5,
				)
			}
		}
	}

	pub fn project_triangle(&self, tri: Triangle3d) -> Triangle3d {
		Triangle3d(
			self.project_point(tri.0),
			self.project_point(tri.1),
			self.project_point(tri.2),
		)
	}
}
