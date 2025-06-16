use std::rc::Rc;

use super::function::Function;

#[derive(Debug, Clone, Copy)]
pub struct Constant(f32);

impl Constant {
	#[must_use]
	pub fn new(c: f32) -> Self {
		Self(c)
	}
}

impl std::fmt::Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Function for Constant {
	fn eval(&self, _x: f32) -> Option<f32> {
		Some(self.0)
	}
}

impl From<Constant> for Rc<dyn Function> {
	fn from(val: Constant) -> Self {
		Rc::new(val)
	}
}

/// Create a constant function.
#[macro_export]
macro_rules! f_const {
	($x:expr) => {
		$crate::functions::Constant::new($x as f32)
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constant() {
		let f = f_const!(5);
		assert_eq!(f.eval(0.0).unwrap(), 5.0);
	}
}
