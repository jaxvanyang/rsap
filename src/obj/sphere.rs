use std::f32::consts::PI;

use crate::Vec3f;

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
	pub center: Vec3f,
	pub radius: f32,
	pub radius2: f32,
	pub color: Vec3f,
}

impl Sphere {
	#[must_use]
	pub fn new(center: Vec3f, radius: f32, color: Vec3f) -> Self {
		assert!(radius >= 0.0);
		Self {
			center,
			radius,
			radius2: radius * radius,
			color,
		}
	}
}

impl Object for Sphere {
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
		let l = self.center - *orig;
		let tca = l.dot(dir);
		let d2 = l.length2() - tca * tca;
		if d2 > self.radius2 {
			return None;
		}
		let thc = (self.radius2 - d2).sqrt();
		let t0 = tca - thc;
		let t1 = tca + thc;

		if t1 < 0.0 {
			return None;
		}

		if t0 > 0.0 {
			return Some(t0);
		}

		Some(t1)
	}

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f {
		let nhit = (*phit - self.center).normalized();
		let x = nhit.z.atan2(nhit.x).abs() / PI;
		let y = nhit.y.asin() / PI + 0.5;
		let scale: f32 = 4.0;
		let pattern = u32::from(((x * scale) % 1.0 > 0.5) ^ ((y * scale) % 1.0 > 0.5)) as f32;

		self.color.mix(&(self.color * 0.8), pattern) * nhit.dot(dir).abs()
	}
}
