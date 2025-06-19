use std::ops;

use crate::Vec3f;

/// 4x4 matrix with `f32` components.
///
/// # Example
///
/// ```
/// # use rsap::Matrix44f;
/// let m = Matrix44f([[0.0; 4]; 4]);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix44f(pub [[f32; 4]; 4]);

impl Matrix44f {
	/// Create a new matrix.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Matrix44f;
	/// let m = Matrix44f::new(
	/// 	0.0, 0.0, 0.0, 0.0,
	/// 	0.0, 0.0, 0.0, 0.0,
	/// 	0.0, 0.0, 0.0, 0.0,
	/// 	0.0, 0.0, 0.0, 0.0,
	/// );
	/// assert_eq!(m, Matrix44f([[0.0; 4]; 4]));
	/// ```
	pub fn new(
		x00: f32,
		x01: f32,
		x02: f32,
		x03: f32,
		x10: f32,
		x11: f32,
		x12: f32,
		x13: f32,
		x20: f32,
		x21: f32,
		x22: f32,
		x23: f32,
		x30: f32,
		x31: f32,
		x32: f32,
		x33: f32,
	) -> Self {
		Self([
			[x00, x01, x02, x03],
			[x10, x11, x12, x13],
			[x20, x21, x22, x23],
			[x30, x31, x32, x33],
		])
	}

	/// Create a new matrix with same components.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Matrix44f;
	/// let m = Matrix44f::sames(1.0);
	/// for i in 0..4 {
	/// 	for j in 0..4 {
	/// 		assert_eq!(m[i][j], 1.0);
	/// 	}
	/// }
	/// ```
	pub fn sames(x: f32) -> Self {
		Self([[x; 4]; 4])
	}

	/// Create a new zero matrix;
	///
	/// # Example
	/// ```
	/// # use rsap::Matrix44f;
	/// let m = Matrix44f::zeros();
	/// for i in 0..4 {
	/// 	for j in 0..4 {
	/// 		assert_eq!(m[i][j], 0.0);
	/// 	}
	/// }
	/// ```
	pub fn zeros() -> Self {
		Self::sames(0.0)
	}

	/// Transpose the matrix.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Matrix44f;
	/// let mut m = Matrix44f::new(
	/// 	1.0, 0.0, 0.0, 0.0,
	/// 	1.0, 1.0, 0.0, 0.0,
	/// 	1.0, 1.0, 1.0, 0.0,
	/// 	1.0, 1.0, 1.0, 1.0,
	/// );
	/// let n = Matrix44f::new(
	/// 	1.0, 1.0, 1.0, 1.0,
	/// 	0.0, 1.0, 1.0, 1.0,
	/// 	0.0, 0.0, 1.0, 1.0,
	/// 	0.0, 0.0, 0.0, 1.0,
	/// );
	/// m.transpose();
	/// assert_eq!(m, n);
	/// ```
	pub fn transpose(&mut self) {
		let m = *self;

		for i in 0..4 {
			for j in 0..4 {
				self[i][j] = m[j][i];
			}
		}
	}

	/// Return a transposed copy of the matrix.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::Matrix44f;
	/// let m = Matrix44f::new(
	/// 	1.0, 0.0, 0.0, 0.0,
	/// 	1.0, 1.0, 0.0, 0.0,
	/// 	1.0, 1.0, 1.0, 0.0,
	/// 	1.0, 1.0, 1.0, 1.0,
	/// );
	/// let n = Matrix44f::new(
	/// 	1.0, 1.0, 1.0, 1.0,
	/// 	0.0, 1.0, 1.0, 1.0,
	/// 	0.0, 0.0, 1.0, 1.0,
	/// 	0.0, 0.0, 0.0, 1.0,
	/// );
	/// assert_eq!(m.transposed(), n);
	/// ```
	pub fn transposed(&self) -> Self {
		let mut m = *self;
		m.transpose();
		m
	}

	/// Multiply a 3D vector using matrix multiplication.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::*;
	/// let m = Matrix44f::new(
	/// 	0.0, 0.0, 1.0, 0.0,
	/// 	0.0, 1.0, 0.0, 0.0,
	/// 	1.0, 0.0, 0.0, 0.0,
	/// 	0.0, 0.0, 0.0, 0.0,
	/// );
	/// let v = Vec3f::new(1.0, 2.0, 3.0);
	/// assert_eq!(m.mul_vector(&v), Vec3f::new(3.0, 2.0, 1.0));
	/// ```
	pub fn mul_vector(&self, vector: &Vec3f) -> Vec3f {
		let x = self[0][0] * vector.x + self[0][1] * vector.y + self[0][2] * vector.z;
		let y = self[1][0] * vector.x + self[1][1] * vector.y + self[1][2] * vector.z;
		let z = self[2][0] * vector.x + self[2][1] * vector.y + self[2][2] * vector.z;

		Vec3f::new(x, y, z)
	}

	/// Multiply a 3D point using CG multiplication.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::*;
	/// let m = Matrix44f::new(
	/// 	0.0, 0.0, 1.0, 1.0,
	/// 	0.0, 1.0, 0.0, 1.0,
	/// 	1.0, 0.0, 0.0, 1.0,
	/// 	0.0, 0.0, 0.0, 1.0,
	/// );
	/// let v = Vec3f::new(1.0, 2.0, 3.0);
	/// assert_eq!(m.mul_point(&v), Vec3f::new(4.0, 3.0, 2.0));
	/// ```
	pub fn mul_point(&self, vector: &Vec3f) -> Vec3f {
		let x = self[0][0] * vector.x + self[0][1] * vector.y + self[0][2] * vector.z + self[0][3];
		let y = self[1][0] * vector.x + self[1][1] * vector.y + self[1][2] * vector.z + self[1][3];
		let z = self[2][0] * vector.x + self[2][1] * vector.y + self[2][2] * vector.z + self[2][3];
		let w = self[3][0] * vector.x + self[3][1] * vector.y + self[3][2] * vector.z + self[3][3];

		if w == 0.0 {
			Vec3f::new(x, y, z)
		} else {
			Vec3f::new(x / w, y / w, z / w)
		}
	}
}

impl Default for Matrix44f {
	fn default() -> Self {
		Matrix44f([
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		])
	}
}

impl ops::Index<usize> for Matrix44f {
	type Output = [f32; 4];

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl ops::IndexMut<usize> for Matrix44f {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl ops::Mul<Vec3f> for Matrix44f {
	type Output = Vec3f;

	fn mul(self, rhs: Vec3f) -> Self::Output {
		self.mul_point(&rhs)
	}
}
