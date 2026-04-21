pub mod camera;
pub mod object;

pub struct Scene {
	pub camera: Camera,
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn new() -> Scene {
		Scene {
			camera: Camera::new(),
			objects: Vec::new(),
		}
	}

	pub fn render_into(&self, frame: &mut Frame) {
		let vector_dimensions: Vector2 = frame.get_dimensions().into();

		// tri, col, dist
		let mut projected_triangles: Vec<(Triangle2d, Color, f64)> = Vec::new();

		for obj in &self.objects {
			for (tri, col) in &obj.triangles {
				let absolute_tri = obj.transform_triangle(*tri);
				let relative_tri = self.camera.transform_triangle(absolute_tri);
				let projected_tri = self.camera.project_triangle(absolute_tri);
				let bounds = projected_tri.get_bounds();
				if bounds.min.x > 1. || bounds.min.y > 1. || bounds.max.x < 0. || bounds.max.y < 0.
				{
					continue;
				}
				let dist = relative_tri.get_centroid().magnitude();
				projected_triangles.push((projected_tri, *col, dist));
			}
		}

		// Lowest dist will be last in the list
		projected_triangles.sort_by(|(_, _, lhs), (_, _, rhs)| rhs.total_cmp(lhs));

		std::io::stdout().flush().unwrap();
		for (tri, col, _) in projected_triangles {
			let screen_tri = Triangle2d(
				tri.0 * vector_dimensions,
				tri.1 * vector_dimensions,
				tri.2 * vector_dimensions,
			);

			frame.draw_tri(screen_tri, col);
		}
		std::io::stdout().flush().unwrap();
	}
}

use std::io::Write;

pub use camera::Camera;
pub use camera::CameraProjection;
pub use object::Object;

use crate::types::Color;
use crate::{
	frame::Frame,
	types::{Triangle2d, Vector2},
};
