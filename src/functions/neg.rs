use std::rc::Rc;

use super::function::Function;

/// Return negative input.
pub struct Negative(Rc<dyn Function>);

impl Negative {
	pub fn new(input: impl Into<Rc<dyn Function>>) -> Self {
		Self(input.into())
	}
}

impl std::fmt::Display for Negative {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "-{}", self.0)
	}
}

impl Function for Negative {
	fn eval(&self, x: f32) -> Option<f32> {
		Some(-self.0.eval(x)?)
	}
}

impl From<Negative> for Rc<dyn Function> {
	fn from(val: Negative) -> Self {
		Rc::new(val)
	}
}

/// Create a function to return negative input.
#[macro_export]
macro_rules! f_neg {
	($input:expr) => {
		$crate::functions::Negative::new($input)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::f_var;

	#[test]
	fn test_neg() {
		let f = f_neg!(f_var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);
	}
}
