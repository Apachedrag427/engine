use std::ops::{Add, Div, Mul};

#[inline]
fn clampf(v: f64, min: f64, max: f64) -> f64 {
	if min > v {
		min
	} else if max < v {
		max
	} else {
		v
	}
}
#[inline]
fn round(n: f64) -> isize {
	(n + 0.5).floor() as isize
}

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
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
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
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x + rhs.x, self.y + rhs.y)
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
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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
	pub fn dot(&self, rhs: Vector3) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
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
}

impl Add<Coordinate2d> for Coordinate2d {
	type Output = Coordinate2d;
	fn add(self, rhs: Coordinate2d) -> Self::Output {
		Coordinate2d::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Into<Coordinate2d> for Vector2 {
	fn into(self) -> Coordinate2d {
		Coordinate2d {
			x: round(self.x),
			y: round(self.y),
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

#[derive(Debug, Clone, Copy)]
pub struct Rect {
	pub position: Vector2,
	pub dimensions: Vector2,
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateRect {
	pub position: Coordinate2d,
	pub dimensions: Coordinate2d
}

impl Into<CoordinateRect> for Rect {
	fn into(self) -> CoordinateRect {
		CoordinateRect { position: self.position.into(), dimensions: self.dimensions.into() }
	}
}

impl Into<Rect> for CoordinateRect {
	fn into(self) -> Rect {
		Rect { position: self.position.into(), dimensions: self.dimensions.into() }
	}
}

#[derive(Debug)]
pub struct Triangle2d {
	pub point1: Vector2,
	pub point2: Vector2,
	pub point3: Vector2,
}

#[derive(Debug)]
pub struct Triangle3d {
	pub point1: Vector3,
	pub point2: Vector3,
	pub point3: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct Color3 {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}
impl Color3 {
	#[inline]
	pub fn new(r: f64, g: f64, b: f64) -> Color3 {
		Color3 { r, g, b }
	}

	#[inline]
	pub fn white() -> Color3 {
		Color3::new(1., 1., 1.)
	}
	#[inline]
	pub fn black() -> Color3 {
		Color3::new(0., 0., 0.)
	}
	#[inline]
	pub fn red() -> Color3 {
		Color3::new(1., 0., 0.)
	}
	#[inline]
	pub fn green() -> Color3 {
		Color3::new(0., 1., 0.)
	}
	#[inline]
	pub fn blue() -> Color3 {
		Color3::new(0., 0., 1.)
	}

	#[inline]
	pub fn invert(&self) -> Color3 {
		Color3 {
			r: 1. - self.r,
			g: 1. - self.g,
			b: 1. - self.b,
		}
	}

	#[inline]
	pub fn from_value(n: f64) -> Color3 {
		Color3 { r: n, g: n, b: n }
	}

	#[inline]
	pub fn get_lightness(&self) -> f64 {
		clampf((0.21 * self.r) + (0.72 * self.g) + (0.07 * self.b), 0., 1.)
	}
}

impl Mul<f64> for Color3 {
	type Output = Color3;

	fn mul(self, rhs: f64) -> Self::Output {
		Color3::new(self.r * rhs, self.g * rhs, self.b * rhs)
	}
}

impl Div<f64> for Color3 {
	type Output = Color3;

	fn div(self, rhs: f64) -> Self::Output {
		Color3::new(self.r / rhs, self.g / rhs, self.b / rhs)
	}
}

impl Default for Color3 {
	fn default() -> Self {
		Color3 {
			r: 0.,
			g: 0.,
			b: 0.,
		}
	}
}

pub enum RasterError {
	IndexOutOfBoundsError,
}

pub struct Frame {
	width: usize,
	height: usize,
	data: Vec<Color3>,
}

impl Frame {
	pub fn new(width: usize, height: usize) -> Frame {
		Frame {
			width,
			height,
			data: vec![Color3::default(); width * height],
		}
	}

	pub fn get_dimensions(&self) -> (usize, usize) {
		(self.width, self.height)
	}

	pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color3> {
		self.data.get(y * self.width + x)
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: Color3) {
		self.data[y * self.width + x] = color;
	}

	// Explanation of Bresenham's Line Algorithm: https://www.youtube.com/watch?v=CceepU1vIKo
	pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color3) {
		if (end.x - start.x).abs() > (end.y - start.y).abs() {
			self.draw_line_horizontal(start, end, color);
		} else {
			self.draw_line_vertical(start, end, color);
		}
	}

	pub fn draw_line_int(&mut self, start: Coordinate2d, end: Coordinate2d, color: Color3) {
		if (end.x - start.x).abs() > (end.y - start.y).abs() {
			self.draw_line_horizontal_int(start, end, color);
		} else {
			self.draw_line_vertical_int(start, end, color);
		}
	}

	fn draw_line_horizontal(&mut self, start: Vector2, end: Vector2, color: Color3) {
		self.draw_line_horizontal_int(start.into(), end.into(), color);
	}

	fn draw_line_horizontal_int(&mut self, mut start: Coordinate2d, mut end: Coordinate2d, color: Color3) {
		if start.x > end.x {
			(start.x, end.x) = (end.x, start.x);
			(start.y, end.y) = (end.y, start.y);
		}

		if start.y == end.y {
			self.fill_rect_int(CoordinateRect { position: start, dimensions: Coordinate2d::new(end.x - start.x, 1) }, color);
		}

		let dx = end.x - start.x;
		let mut dy = end.y - start.y;

		let dir = if dy < 0 { -1 } else { 1 };
		dy *= dir;

		if dx != 0 {
			let mut y = start.y;
			let mut p = 2 * dy - dx;
			for x in 0..=dx {
				self.set_pixel(start.x as usize + x as usize, y as usize, color);
				if p >= 0 {
					y += dir;
					p -= 2 * dx;
				}
				p += 2 * dy;
			}
		}
	}

	fn draw_line_vertical(&mut self, start: Vector2, end: Vector2, color: Color3) {
		self.draw_line_vertical_int(start.into(), end.into(), color);
	}

	fn draw_line_vertical_int(&mut self, mut start: Coordinate2d, mut end: Coordinate2d, color: Color3) {
		if start.y > end.y {
			(start.x, end.x) = (end.x, start.x);
			(start.y, end.y) = (end.y, start.y);
		}

		if start.x == end.x {
			self.fill_rect_int(CoordinateRect { position: start, dimensions: Coordinate2d::new(1, end.y - start.y)}, color);
		}

		let mut dx = end.x - start.x;
		let dy = end.y - start.y;

		let dir = if dx < 0 { -1 } else { 1 };
		dx *= dir;

		if dy != 0 {
			let mut x = start.x;
			let mut p = 2 * dx - dy;
			for y in 0..=dy {
				self.set_pixel(x as usize, start.y as usize + y as usize, color);
				if p >= 0 {
					x += dir;
					p -= 2 * dy;
				}
				p += 2 * dx;
			}
		}
	}

	pub fn fill_rect(&mut self, rect: Rect, color: Color3) {
		self.fill_rect_int(rect.into(), color);
	}

	pub fn fill_rect_int(&mut self, rect: CoordinateRect, color: Color3) {
		let start: Coordinate2d = rect.position.into();
		let mut end: Coordinate2d = (rect.position + rect.dimensions).into();

		// A scale of 1 means 0 offset.
		end.x -= 1;
		end.y -= 1;

		if start.x == end.x {
			let mut i = start.y * (self.width as isize) + start.x;
			for _y in start.y..=end.y {
				self.data[i as usize] = color;

				// Go down a row
				i += self.width as isize;
			}
		}

		let mut i;
		for y in start.y..=end.y {
			i = y * (self.width as isize) + start.x;
			for _x in start.x..=end.x {
				self.data[i as usize] = color;
				i += 1;
			}
		}
	}

	pub fn clear(&mut self, color: Color3) {
		self.data.fill(color);
	}

	pub fn callback_fill<T: Fn(usize, usize) -> Color3>(&mut self, callback: T) {
		// Use a separate index here to avoid having to recompute it for every pixel
		// The caveat is that I **MUST** loop row by row for the index to line up
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.data[i] = callback(x, y);
				i += 1;
			}
		}
	}

	pub fn callback_update<T: Fn(usize, usize, Color3) -> Color3>(&mut self, callback: T) {
		// Same index rules as callback_fill
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.data[i] = callback(x, y, self.data[i]);
				i += 1;
			}
		}
	}
}

pub enum FrameItem {
	Pixel(usize, usize, Color3),
	LineEnd,
}

pub struct FrameIter {
	data: Vec<Color3>,

	// Height is unnecessary because `Vec.get` will catch it if it goes down too many rows.
	width: usize,

	current_x: usize,
	current_y: usize,
	current_i: usize,

	// Functions as a flag that, when set, will override the next iteration to return FrameItem::LineEnd
	on_line_end: bool,
}

impl Iterator for FrameIter {
	type Item = FrameItem;

	fn next(&mut self) -> Option<Self::Item> {
		if self.on_line_end {
			self.on_line_end = false;
			return Some(FrameItem::LineEnd);
		}

		let current = match self.data.get(self.current_i) {
			Some(color) => color,
			None => return None,
		};

		let current_x = self.current_x;
		let current_y = self.current_y;

		self.current_i += 1;
		self.current_x += 1;
		if self.current_x >= self.width {
			self.current_x = 0;
			self.current_y += 1;

			self.on_line_end = true;
		}

		Some(FrameItem::Pixel(current_x, current_y, current.clone()))
	}
}

impl IntoIterator for Frame {
	type Item = FrameItem;
	type IntoIter = FrameIter;

	fn into_iter(self) -> Self::IntoIter {
		FrameIter {
			data: self.data,
			width: self.width,
			current_x: 0,
			current_y: 0,
			current_i: 0,
			on_line_end: false,
		}
	}
}
