use crate::consts::EPS;

#[inline]
#[must_use]
pub fn is_equal(a: f32, b: f32) -> bool {
	(a - b).abs() < EPS
}

#[inline]
#[must_use]
pub fn is_zero(x: f32) -> bool {
	is_equal(x, 0.0)
}
