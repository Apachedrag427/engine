use rasterizer::{Frame, FrameItem};

pub trait Renderer {
	fn begin_rendering(&mut self);
	fn end_rendering(&mut self);
	fn prepare_for_next_frame(&mut self, frame: &Frame);
	fn render_frame(&mut self, frame: Frame);
}

pub mod stdout_grayscale_ascii {
	use super::*;
	static LIGHTNESS_CHARACTERS: [char; 10] = [' ', '.', ':', '-', '+', '=', '*', '@', '%', '#'];

	pub struct StdoutGrayscaleAsciiRenderer;

	impl Renderer for StdoutGrayscaleAsciiRenderer {
		fn begin_rendering(&mut self) {
			// Hide cursor
			print!("\x1b[?25l");
			// Return cursor to "home position"
			print!("\x1b[1;1H");
		}

		fn end_rendering(&mut self) {
			// Show cursor
			print!("\x1b[?25h");
		}

		fn prepare_for_next_frame(&mut self, _frame: &Frame) {
			// Return cursor to "home position"
			print!("\x1b[1;1H");
		}

		fn render_frame(&mut self, frame: Frame) {
			let mut result = String::new();
			for item in frame {
				match item {
					FrameItem::Pixel(_x, _y, color) => {
						let lightness = color.get_lightness();
						result.push(
							LIGHTNESS_CHARACTERS[(lightness
								* ((LIGHTNESS_CHARACTERS.len() - 1) as f64)
								+ 0.5)
								.floor() as usize],
						);
					}
					FrameItem::LineEnd => result.push('\n'),
				}
			}
			println!("{}", result);
		}
	}
}

pub mod stdout_color_ansi {
	use super::*;

	pub struct StdoutColorAnsiRenderer;

	impl Renderer for StdoutColorAnsiRenderer {
		fn begin_rendering(&mut self) {
			// Hide cursor
			print!("\x1b[?25l");
			// Return cursor to "home position"
			print!("\x1b[1;1H");
		}

		fn end_rendering(&mut self) {
			// Show cursor
			print!("\x1b[?25h");
		}

		fn prepare_for_next_frame(&mut self, _frame: &Frame) {
			// Return cursor to "home position"
			print!("\x1b[1;1H");
		}

		fn render_frame(&mut self, frame: Frame) {
			let mut result = String::new();
			for item in frame {
				match item {
					FrameItem::Pixel(_x, _y, color) => {
						let r = (color.r * 255.) as i32;
						let g = (color.g * 255.) as i32;
						let b = (color.b * 255.) as i32;
						result.push_str(format!("\x1b[48;2;{r};{g};{b}m  ").as_str())
					}
					FrameItem::LineEnd => result.push_str("\x1b[0m\n"),
				}
			}
			println!("{}", result);
		}
	}
}

pub mod kitty_graphics {
	use super::*;
	use base64::{Engine as _, engine::general_purpose};

	pub struct KittyGraphicsRenderer;

	impl Renderer for KittyGraphicsRenderer {
		fn begin_rendering(&mut self) {
			// Hide cursor
			print!("\x1b[?25l");
			// Return cursor to "home position"
			print!("\x1b[1;1H");
		}

		fn end_rendering(&mut self) {
			// Show cursor
			print!("\x1b[?25h");
		}

		fn prepare_for_next_frame(&mut self, _frame: &Frame) {
			// Return cursor to "home position"
			print!("\x1b[1;1H");
			// Delete all kitty images
			println!("\x1bG_a=d,d=A;\x1b\\");
		}

		fn render_frame(&mut self, frame: Frame) {
			let dimensions = frame.get_dimensions();
			let mut data: Vec<u8> = Vec::new();
			for item in frame {
				match item {
					FrameItem::Pixel(_x, _y, color) => {
						let r = (color.r * 255.) as u8;
						let g = (color.g * 255.) as u8;
						let b = (color.b * 255.) as u8;
						data.push(r);
						data.push(g);
						data.push(b);
					}
					FrameItem::LineEnd => (),
				}
			}
			print!(
				"\x1b_Ga=T,f=24,s={},v={},c=60,r=30;{}\x1b\\",
				dimensions.0,
				dimensions.1,
				general_purpose::STANDARD.encode(&data[..])
			);
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
		}
	}
}
