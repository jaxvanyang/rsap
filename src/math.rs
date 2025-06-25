use std::f32::consts::PI;

/// convert radian to degree.
///
/// # Example
///
/// ```
/// # use rsap::math::*;
/// # use std::f32::consts::PI;
/// assert_eq!(rad2deg(PI), 180.0);
/// ```
#[must_use]
#[inline]
pub fn rad2deg(rad: f32) -> f32 {
	rad / PI * 180.0
}

/// convert degree to radian.
///
/// # Example
///
/// ```
/// # use rsap::math::*;
/// # use std::f32::consts::PI;
/// assert_eq!(deg2rad(180.0), PI);
/// ```
#[must_use]
#[inline]
pub fn deg2rad(deg: f32) -> f32 {
	deg / 180.0 * PI
}

/// Calculate factorial of a number.
///
/// # Examples
///
/// ```
/// # use rsap::math::*;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(2), 2);
/// assert_eq!(factorial(6), 720);
/// ```
#[must_use]
#[inline]
pub fn factorial(n: u32) -> u32 {
	if n <= 1 {
		1
	} else {
		n * factorial(n - 1)
	}
}
