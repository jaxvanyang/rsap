use crate::Vec3f;

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
	pub vertices: [Vec3f; 3],
	pub colors: [Vec3f; 3],
}

impl Triangle {
	pub fn new(a: Vec3f, b: Vec3f, c: Vec3f, ca: Vec3f, cb: Vec3f, cc: Vec3f) -> Self {
		Self {
			vertices: [a, b, c],
			colors: [ca, cb, cc],
		}
	}
}

impl Default for Triangle {
	fn default() -> Self {
		let a = Vec3f::new(1.0, 0.0, 0.0);
		let b = Vec3f::new(0.0, 1.0, 0.0);
		let c = Vec3f::new(0.0, 0.0, 1.0);

		Self::new(a, b, c, a, b, c)
	}
}

impl Object for Triangle {
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
		let ab = self.vertices[1] - self.vertices[0];
		let ac = self.vertices[2] - self.vertices[0]; // dbg!(ab.cross(&ac));
		let n = ab.cross(&ac).normalized();
		let denom = n.dot(dir);
		if denom.abs() < 1e-6 {
			return None;
		}

		let l = self.vertices[0] - *orig;
		let t = n.dot(&l) / denom;
		if t < 0.0 {
			return None;
		}

		let phit = *orig + *dir * t;

		for i in 0..3 {
			let ab = self.vertices[(i + 1) % 3] - self.vertices[i % 3];
			let ap = phit - self.vertices[i % 3];
			let m = ab.cross(&ap);

			if n.dot(&m) < 0.0 {
				return None;
			}
		}

		Some(t)
	}

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f {
		let ab = self.vertices[1] - self.vertices[0];
		let ac = self.vertices[2] - self.vertices[0];
		let n = ab.cross(&ac);
		let n2 = n.length2();
		let ap = *phit - self.vertices[0];

		let w = ab.cross(&ap).dot(&n) / n2;
		assert!(0.0 <= w && w <= 1.0);
		let v = ap.cross(&ac).dot(&n) / n2;
		assert!(0.0 <= v && v <= 1.0);
		let u = 1.0 - w - v;
		assert!(0.0 <= u && u <= 1.0);

		self.colors[0] * u + self.colors[1] * v + self.colors[2] * w
	}
}
