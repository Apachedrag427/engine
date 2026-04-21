use crate::types::{CFrame, Triangle2d, Triangle3d, Vector2, Vector3};

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
	// easier than aspect ratio alone
	pub screen_dimensions: Vector2,
	pub projection: CameraProjection,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			cframe: CFrame::identity(),
			scale: 1.,
			fov: 70.,
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

	pub fn project_point(&self, point: Vector3) -> Vector2 {
		match self.projection {
			CameraProjection::Orthographic => {
				let relative_point =
					(point - self.cframe.position).rotate(self.cframe.rotation.inverse());

				let plane_corner = self.screen_dimensions.normalize() * self.scale;
				let start_corner = Vector2::new(-plane_corner.x, plane_corner.y);
				let end_corner = Vector2::new(plane_corner.x, -plane_corner.y);

				Vector2::new(
					(relative_point.x - start_corner.x) / (end_corner.x - start_corner.x),
					(relative_point.y - start_corner.y) / (end_corner.y - start_corner.y),
				)
			}
			_ => todo!(),
		}
	}

	pub fn project_triangle(&self, tri: Triangle3d) -> Triangle2d {
		match self.projection {
			CameraProjection::Orthographic => Triangle2d(
				self.project_point(tri.0),
				self.project_point(tri.1),
				self.project_point(tri.2),
			),
			_ => todo!(),
		}
	}
}
