use crate::deg2rad;
use crate::Matrix44f;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderOpt {
	pub width: usize,
	pub height: usize,
	pub fov: f32,
	pub camera_to_world: Matrix44f,
}

impl RenderOpt {
	pub fn new(width: usize, height: usize, fov: f32, camera_to_world: Matrix44f) -> Self {
		Self {
			width,
			height,
			fov,
			camera_to_world,
		}
	}

	/// Return the ratio of width to height.
	pub fn aspect_ratio(&self) -> f32 {
		self.width as f32 / self.height as f32
	}

	/// Return the tangent of half of the fov.
	///
	/// # Example
	///
	/// ```
	/// # use rsap::option::RenderOpt;
	/// # let mut option = RenderOpt::default();
	/// option.fov = 90.0;
	/// assert_eq!(option.angle(), 1.0);
	/// ```
	pub fn angle(&self) -> f32 {
		deg2rad(self.fov / 2.0).tan()
	}
}

impl Default for RenderOpt {
	fn default() -> Self {
		Self {
			width: 800,
			height: 600,
			fov: 120.0,
			camera_to_world: Matrix44f::default(),
		}
	}
}
