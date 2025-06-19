use crate::Vec3f;

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
	pub center: Vec3f,
	pub normal: Vec3f,
	pub color: Vec3f,
}

impl Plane {
	#[must_use]
	pub fn new(center: Vec3f, normal: Vec3f, color: Vec3f) -> Self {
		Self {
			center,
			normal,
			color,
		}
	}
}

impl Object for Plane {
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
		let denom = dir.dot(&self.normal);
		if denom < 1e-6 {
			return None;
		}

		let l = self.center - *orig;
		let t = l.dot(&self.normal) / denom;

		Some(t)
	}

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f {
		let scale = 1.0;
		let l = *phit - self.center;
		let x_pattern = l.x.abs() * scale % 1.0 > 0.5;
		let y_pattern = l.y.abs() * scale % 1.0 > 0.5;
		let pattern = (x_pattern ^ (l.x < 0.0)) ^ (y_pattern ^ (l.y < 0.0));

		self.color
			.mix(&(self.color * 0.8), u32::from(pattern) as f32)
			* self.normal.dot(dir)
	}
}
