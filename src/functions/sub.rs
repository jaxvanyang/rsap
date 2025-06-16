use std::rc::Rc;

use super::function::Function;

pub struct Substract {
	left: Rc<dyn Function>,
	right: Rc<dyn Function>,
}

impl Substract {
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

impl std::fmt::Display for Substract {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} - {}", self.left, self.right)
	}
}

impl Function for Substract {
	fn is_x_valid(&self, x: f32) -> bool {
		self.left.is_x_valid(x) && self.right.is_x_valid(x)
	}

	fn eval(&self, x: f32) -> Option<f32> {
		Some(self.left.eval(x)? - self.right.eval(x)?)
	}
}

impl From<Substract> for Rc<dyn Function> {
	fn from(val: Substract) -> Self {
		Rc::new(val)
	}
}

/// Create a function to subtract two inputs.
#[macro_export]
macro_rules! f_sub {
	($left:expr, $right:expr) => {
		$crate::functions::Substract::new($left, $right)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::f_const;

	#[test]
	fn test_minus() {
		let f = f_sub!(f_const!(2), f_const!(1));
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
	}
}
