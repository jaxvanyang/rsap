use std::rc::Rc;

use super::function::Function;

pub struct Multiply {
	left: Rc<dyn Function>,
	right: Rc<dyn Function>,
}

impl Multiply {
	pub fn new<L, R>(left: L, right: R) -> Self
	where
		L: Into<Rc<dyn Function>>,
		R: Into<Rc<dyn Function>>,
	{
		Self {
			left: left.into(),
			right: right.into(),
		}
	}
}

impl std::fmt::Display for Multiply {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} * {}", self.left, self.right)
	}
}

impl Function for Multiply {
	fn is_x_valid(&self, x: f32) -> bool {
		self.left.is_x_valid(x) && self.right.is_x_valid(x)
	}

	fn eval(&self, x: f32) -> Option<f32> {
		Some(self.left.eval(x)? * self.right.eval(x)?)
	}
}

impl From<Multiply> for Rc<dyn Function> {
	fn from(val: Multiply) -> Self {
		Rc::new(val)
	}
}

/// Create a function to multiply two inputs.
#[macro_export]
macro_rules! f_mul {
	($left:expr, $right:expr) => {
		$crate::functions::Multiply::new($left, $right)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::f_const;

	#[test]
	fn test_mul() {
		let f = f_mul!(f_const!(1), f_const!(2));
		assert_eq!(f.eval(0.0).unwrap(), 2.0);
	}
}
