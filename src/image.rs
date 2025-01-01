use crate::Vec3f;
use std::ops;

#[derive(Debug)]
pub struct Image {
	pub width: usize,
	pub height: usize,
	pub buffer: Vec<Vec3f>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			buffer: vec![Vec3f::zeros(); width * height],
		}
	}

	/// Return a byte vector of the image in P6 PPM format.
	pub fn to_ppm(&self) -> Vec<u8> {
		let mut ppm = Vec::new();
		ppm.extend_from_slice(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes());
		for pixel in &self.buffer {
			ppm.extend_from_slice(&[
				(pixel.x.min(1.0) * 255.0) as u8,
				(pixel.y.min(1.0) * 255.0) as u8,
				(pixel.z.min(1.0) * 255.0) as u8,
			]);
		}
		ppm
	}
}

impl ops::Index<usize> for Image {
	type Output = Vec3f;

	fn index(&self, index: usize) -> &Self::Output {
		&self.buffer[index]
	}
}

impl ops::IndexMut<usize> for Image {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.buffer[index]
	}
}
