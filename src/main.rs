use rasterizer::frame::Frame;
use rasterizer::types::{Color, Coordinate2d, Triangle2d, Vector2};

use rasterizer::render;
use rasterizer::render::RenderBackend;

fn main() {
	let width = 800;
	let height = 800;

	let middle_x = width as f64 / 2.;
	let middle_y = height as f64 / 2.;

	let mut renderer = render::KittySHMRenderer;
	renderer.begin_rendering();

	loop {
		let time = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs_f64()
			/ 2.;

		let mut frame = Frame::new(width, height);

		let mut mini_frame = Frame::new(100, 100);
		mini_frame.callback_fill(|x, y| {
			if (y + x) / 8 % 2 == 0 {
				Color::white()
			} else {
				Color::transparent()
			}
		});

		// let bg_color =
		// 	Color3::new(f64::cos(time) * 0.5 + 0.5, f64::sin(time) * 0.5 + 0.5, 0.) * 0.65;

		frame.clear(Color::black());
		frame.draw_frame_int(Coordinate2d::one(), mini_frame);

		let tri = Triangle2d(
			Vector2::new(
				middle_x + f64::cos(time) * 30.,
				middle_y + f64::sin(time) * 30.,
			),
			Vector2::new(middle_x + f64::cos(time) * 300., middle_y),
			Vector2::new(middle_x, middle_y + f64::sin(time) * 300.),
		);

		frame.draw_tri(tri, Color::green());

		frame.draw_wireframe_tri(tri, Color::red());

		// let arrow_length = 300.;
		// let arrow_leaf_length = 50.;
		// let arrow_leaf_offset = 0.5;

		// let arrow_color = bg_color.invert();

		// let arrow_point = Vector2::new(
		// 	middle_x + f64::cos(time) * arrow_length,
		// 	middle_y + f64::sin(time) * arrow_length,
		// );

		// frame.draw_line(Vector2::new(middle_x, middle_y), arrow_point, arrow_color);
		// frame.draw_line(
		// 	arrow_point,
		// 	Vector2::new(
		// 		arrow_point.x - arrow_leaf_length * f64::cos(time + arrow_leaf_offset),
		// 		arrow_point.y - arrow_leaf_length * f64::sin(time + arrow_leaf_offset),
		// 	),
		// 	arrow_color,
		// );
		// frame.draw_line(
		// 	arrow_point,
		// 	Vector2::new(
		// 		arrow_point.x - arrow_leaf_length * f64::cos(time - arrow_leaf_offset),
		// 		arrow_point.y - arrow_leaf_length * f64::sin(time - arrow_leaf_offset),
		// 	),
		// 	arrow_color,
		// );

		// frame.fill_rect(
		// 	Rect {
		// 		position: Vector2::new(150., 150.),
		// 		dimensions: Vector2::new(100., 1.),
		// 	},
		// 	Color3::black(),
		// );

		renderer.prepare_for_next_frame(&frame);
		renderer.render_frame(frame);

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
