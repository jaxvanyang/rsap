use crate::Vec3f;

use super::Object;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
	pub min: Vec3f,
	pub max: Vec3f,
	pub center: Vec3f,
	pub color: Vec3f,
}

impl AABB {
	#[must_use]
	pub fn new(min: Vec3f, max: Vec3f, color: Vec3f) -> Self {
		assert!(min.x <= max.x);
		assert!(min.y <= max.y);
		assert!(min.z <= max.z);

		Self {
			min,
			max,
			center: (min + max) / 2.0,
			color,
		}
	}
}

impl Object for AABB {
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
		// TODO:
		// let invdir = 1.0 / dir;

		let (mut tmin, mut tmax) = if dir.x >= 0.0 {
			((self.min.x - orig.x) / dir.x, (self.max.x - orig.x) / dir.x)
		} else {
			((self.max.x - orig.x) / dir.x, (self.min.x - orig.x) / dir.x)
		};

		let (ty0, ty1) = if dir.y >= 0.0 {
			((self.min.y - orig.y) / dir.y, (self.max.y - orig.y) / dir.y)
		} else {
			((self.max.y - orig.y) / dir.y, (self.min.y - orig.y) / dir.y)
		};

		if tmin > ty1 || tmax < ty0 {
			return None;
		}

		if ty0 > tmin {
			tmin = ty0;
		}
		if ty1 < tmax {
			tmax = ty1;
		}

		let (tz0, tz1) = if dir.z >= 0.0 {
			((self.min.z - orig.z) / dir.z, (self.max.z - orig.z) / dir.z)
		} else {
			((self.max.z - orig.z) / dir.z, (self.min.z - orig.z) / dir.z)
		};

		if tmin > tz1 || tmax < tz0 {
			return None;
		}

		if tz0 > tmin {
			tmin = tz0;
		}
		if tz1 < tmax {
			tmax = tz1;
		}

		assert!(tmin <= tmax);

		if tmax < 0.0 {
			None
		} else if tmin < 0.0 {
			Some(tmax)
		} else {
			Some(tmin)
		}
	}

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f {
		let scale = 1.0;
		let x_pattern = (phit.x < 0.0) ^ (phit.x.abs() * scale % 1.0 > 0.5);
		let y_pattern = (phit.y < 0.0) ^ (phit.y.abs() * scale % 1.0 > 0.5);
		let z_pattern = (phit.z < 0.0) ^ (phit.z.abs() * scale % 1.0 > 0.5);
		let pattern = x_pattern ^ y_pattern ^ z_pattern;

		let dx0 = (phit.x - self.min.x).abs();
		let dx1 = (phit.x - self.max.x).abs();
		let dy0 = (phit.y - self.min.y).abs();
		let dy1 = (phit.y - self.max.y).abs();
		let dz0 = (phit.z - self.min.z).abs();
		let dz1 = (phit.z - self.max.z).abs();
		let dmin = [dx0, dx1, dy0, dy1, dz0, dz1]
			.into_iter()
			.reduce(f32::min)
			.unwrap();

		let nhit = if dmin == dx0 || dmin == dx1 {
			Vec3f::new(1.0, 0.0, 0.0)
		} else if dmin == dy0 || dmin == dy1 {
			Vec3f::new(0.0, 1.0, 0.0)
		} else if dmin == dz0 || dmin == dz1 {
			Vec3f::new(0.0, 0.0, 1.0)
		} else {
			panic!("hit point ({phit:?}) not on the box ({self:?}");
		};

		self.color
			.mix(&(self.color * 0.8), u32::from(pattern) as f32)
			* nhit.dot(dir).abs()
	}
}
