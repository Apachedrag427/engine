use crate::types::{CFrame, Color, Triangle3d, Vector3};

pub struct Object {
	pub cframe: CFrame,
	pub scale: Vector3,

	// always in object space
	pub triangles: Vec<(Triangle3d, Color)>,
}

impl Object {
	pub fn new() -> Object {
		Object {
			cframe: CFrame::identity(),
			scale: Vector3::one(),
			triangles: Vec::new(),
		}
	}

	pub fn square() -> Object {
		let mut obj = Object::new();
		obj.triangles.push((
			Triangle3d(
				Vector3::new(0.5, -0.5, 0.),
				Vector3::new(-0.5, -0.5, 0.),
				Vector3::new(-0.5, 0.5, 0.),
			),
			Color::green(),
		));
		obj.triangles.push((
			Triangle3d(
				Vector3::new(-0.5, 0.5, 0.),
				Vector3::new(0.5, 0.5, 0.),
				Vector3::new(0.5, -0.5, 0.),
			),
			Color::red(),
		));
		obj
	}

	pub fn pyramid() -> Object {
		let mut obj = Object::new();
		let up = 0.5;
		let down = -0.5;
		let north = -0.5;
		let south = 0.5;
		let east = 0.5;
		let west = -0.5;
		let tip = Vector3::new(0., up, 0.);
		let ne_corner = Vector3::new(east, down, north);
		let nw_corner = Vector3::new(west, down, north);
		let se_corner = Vector3::new(east, down, south);
		let sw_corner = Vector3::new(west, down, south);

		// bottom
		obj.triangles.push((
			Triangle3d(se_corner, ne_corner, nw_corner),
			Color::new(1., 1., 0.),
		));
		obj.triangles.push((
			Triangle3d(nw_corner, sw_corner, se_corner),
			Color::new(0., 1., 1.),
		));

		// top
		obj.triangles
			.push((Triangle3d(nw_corner, ne_corner, tip), Color::red()));
		obj.triangles
			.push((Triangle3d(sw_corner, nw_corner, tip), Color::green()));
		obj.triangles
			.push((Triangle3d(se_corner, sw_corner, tip), Color::blue()));
		obj.triangles
			.push((Triangle3d(ne_corner, se_corner, tip), Color::white()));

		obj
	}

	pub fn transform_point(&self, point: Vector3) -> Vector3 {
		(point * self.scale)
			.rotate(self.cframe.rotation)
			.translate(self.cframe.position)
	}

	pub fn transform_triangle(&self, tri: Triangle3d) -> Triangle3d {
		Triangle3d(
			self.transform_point(tri.0),
			self.transform_point(tri.1),
			self.transform_point(tri.2),
		)
	}
}
