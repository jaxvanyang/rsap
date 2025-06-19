use crate::Vec3f;

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Disk {
	pub center: Vec3f,
	pub normal: Vec3f,
	pub radius: f32,
	pub radius2: f32,
	pub color: Vec3f,
}

impl Disk {
	#[must_use]
	pub fn new(center: Vec3f, normal: Vec3f, radius: f32, color: Vec3f) -> Self {
		Self {
			center,
			normal,
			radius,
			radius2: radius * radius,
			color,
		}
	}
}

impl Object for Disk {
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
		let denom = dir.dot(&self.normal);
		if denom.abs() < 1e-6 {
			return None;
		}

		let l = self.center - *orig;
		let t = l.dot(&self.normal) / denom;
		if t < 0.0 {
			return None;
		}

		let phit = *orig + *dir * t;
		let d2 = (phit - self.center).length2();

		if d2 > self.radius2 {
			None
		} else {
			Some(t)
		}
	}

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f {
		let scale = 1.0;
		let d = (*phit - self.center).length();
		let pattern = d * scale % 1.0 > 0.5;

		self.color
			.mix(&(self.color * 0.8), u32::from(pattern) as f32)
			* self.normal.dot(dir).abs()
	}
}
