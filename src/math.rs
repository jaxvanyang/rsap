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
pub fn deg2rad(deg: f32) -> f32 {
	deg / 180.0 * PI
}
