pub mod aabb;
pub mod disk;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub use crate::aabb::*;
pub use crate::disk::*;
pub use crate::plane::*;
pub use crate::sphere::*;
pub use crate::triangle::*;
use crate::Vec3f;

pub trait Object {
	/// Return the t parameter if the ray intersect the object.
	fn intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32>;

	fn shade(&self, dir: &Vec3f, phit: &Vec3f) -> Vec3f;
}
