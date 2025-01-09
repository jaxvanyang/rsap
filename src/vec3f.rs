use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3f {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3f {
	/// Create a new vector.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let v = Vec3f::new(1.0, 2.0, 3.0);
	/// ```
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	/// Create a new vector with same components.
	pub fn sames(x: f32) -> Self {
		Self::new(x, x, x)
	}

	/// Create a new zero vector.
	pub fn zeros() -> Self {
		Self::sames(0.0)
	}

	/// Create a new vector whose components are all one.
	pub fn ones() -> Self {
		Self::sames(1.0)
	}

	/// Return the dot product with the `other` vector.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let u = Vec3f::new(1.0, 2.0, 3.0);
	/// let v = Vec3f::new(1.0, 1.0, -1.0);
	/// assert_eq!(u.dot(&v), 0.0);
	/// ```
	pub fn dot(&self, other: &Self) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	/// Return the cross product with the `other` vector.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let i = Vec3f::new(1.0, 0.0, 0.0);
	/// let j = Vec3f::new(0.0, 1.0, 0.0);
	/// let k = Vec3f::new(0.0, 0.0, 1.0);
	/// assert_eq!(i.cross(&j), k);
	pub fn cross(&self, other: &Self) -> Self {
		let x = self.y * other.z - self.z * other.y;
		let y = self.z * other.x - self.x * other.z;
		let z = self.x * other.y - self.y * other.x;

		Self { x, y, z }
	}

	/// Return the square of the vector's length.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let v = Vec3f::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.length2(), 14.0);
	/// ```
	pub fn length2(&self) -> f32 {
		self.dot(self)
	}

	/// Return the vector's length.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let v = Vec3f::new(1.0, 2.0, 2.0);
	/// assert_eq!(v.length(), 3.0);
	/// ```
	pub fn length(&self) -> f32 {
		self.length2().sqrt()
	}

	/// Normalize the vector.
	///
	/// # Examples
	///
	/// ## Zero vector
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let mut v = Vec3f::zeros();
	/// v.normalize();
	/// assert_eq!(v.length(), 0.0);
	/// ```
	///
	/// ## Non-zero vector
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let mut v = Vec3f::new(1.0, 2.0, 2.0);
	/// v.normalize();
	/// assert_eq!(v.length(), 1.0);
	/// ```
	pub fn normalize(&mut self) -> &mut Self {
		let len2 = self.length2();

		if len2 > 0.0 {
			let inv_len = 1.0 / len2.sqrt();
			self.x *= inv_len;
			self.y *= inv_len;
			self.z *= inv_len;
		}

		self
	}

	/// Return a normalized copy of the vector.
	///
	/// # Examples
	///
	/// ## Zero vector
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let u = Vec3f::zeros();
	/// let v = u.normalized();
	/// assert_eq!(v.length(), 0.0);
	/// ```
	///
	/// ## Non-zero vector
	///
	/// ```
	/// # use rsap::Vec3f;
	/// let u = Vec3f::new(1.0, 2.0, 2.0);
	/// let v = u.normalized();
	/// assert_eq!(v.length(), 1.0);
	/// ```
	pub fn normalized(&self) -> Self {
		let mut out = self.clone();
		out.normalize();
		out
	}

	/// Return mix of x (self) and y, i.e. `(1 - k) * x + k * y`.
	pub fn mix(&self, y: &Vec3f, k: f32) -> Vec3f {
		*self * (1.0 - k) + *y * k
	}
}

impl fmt::Display for Vec3f {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl ops::Neg for Vec3f {
	type Output = Self;

	fn neg(self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl ops::Add for Vec3f {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::AddAssign for Vec3f {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

impl ops::Sub for Vec3f {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl ops::SubAssign for Vec3f {
	fn sub_assign(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
	}
}

impl ops::Mul for Vec3f {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Self {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl ops::Mul<f32> for Vec3f {
	type Output = Self;

	fn mul(self, other: f32) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
}

impl ops::MulAssign for Vec3f {
	fn mul_assign(&mut self, other: Self) {
		self.x *= other.x;
		self.y *= other.y;
		self.z *= other.z;
	}
}

impl ops::MulAssign<f32> for Vec3f {
	fn mul_assign(&mut self, other: f32) {
		self.x *= other;
		self.y *= other;
		self.z *= other;
	}
}

impl ops::Div for Vec3f {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		Self {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}

impl ops::Div<f32> for Vec3f {
	type Output = Self;

	fn div(self, other: f32) -> Self {
		Self {
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

impl ops::DivAssign for Vec3f {
	fn div_assign(&mut self, other: Self) {
		self.x /= other.x;
		self.y /= other.y;
		self.z /= other.z;
	}
}

impl ops::DivAssign<f32> for Vec3f {
	fn div_assign(&mut self, other: f32) {
		self.x /= other;
		self.y /= other;
		self.z /= other;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		let a = Vec3f::sames(1.0);
		let b = Vec3f::sames(2.0);
		assert_eq!(a + b, Vec3f::sames(3.0));
	}

	#[test]
	fn test_sub() {
		let a = Vec3f::zeros();
		let b = Vec3f::sames(1.0);
		assert_eq!(a - b, -b);
	}

	#[test]
	fn test_mul() {
		let a = Vec3f::zeros();
		let b = Vec3f::new(1.0, 2.0, 3.0);
		assert_eq!(a * b, Vec3f::zeros());
		assert_eq!(a * 2.0, Vec3f::zeros());
	}

	#[test]
	fn test_div() {
		let a = Vec3f::zeros();
		let b = Vec3f::new(1.0, 2.0, 3.0);
		assert_eq!(a / b, Vec3f::zeros());
		assert_eq!(a / 2.0, Vec3f::zeros());
	}
}
