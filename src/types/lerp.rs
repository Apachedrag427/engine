pub trait Lerp<T> {
	type Output;
	fn lerp(&self, rhs: T, a: f64) -> Self::Output;
}

macro_rules! impl_num_lerp {
	($a:ident, $b:ident) => {
		impl Lerp<$b> for $a {
			type Output = $a;
			fn lerp(&self, rhs: $b, a: f64) -> Self::Output {
				self + (a * (rhs as $a - self) as f64) as $a
			}
		}
	};
}

impl_num_lerp!(f64, f64);
impl_num_lerp!(f64, i32);
impl_num_lerp!(f64, isize);

impl_num_lerp!(i32, f64);
impl_num_lerp!(i32, i32);
impl_num_lerp!(i32, isize);

impl_num_lerp!(isize, f64);
impl_num_lerp!(isize, i32);
impl_num_lerp!(isize, isize);
