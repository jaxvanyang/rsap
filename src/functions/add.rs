use std::rc::Rc;

use super::function::Function;

pub struct Add {
	left: Rc<dyn Function>,
	right: Rc<dyn Function>,
}

impl Add {
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

impl std::fmt::Display for Add {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} + {}", self.left, self.right)
	}
}

impl Function for Add {
	fn is_x_valid(&self, x: f32) -> bool {
		self.left.is_x_valid(x) && self.right.is_x_valid(x)
	}

	fn eval(&self, x: f32) -> Option<f32> {
		Some(self.left.eval(x)? + self.right.eval(x)?)
	}
}

impl From<Add> for Rc<dyn Function> {
	fn from(val: Add) -> Self {
		Rc::new(val)
	}
}

/// Create a function to add two inputs.
#[macro_export]
macro_rules! f_add {
	($left:expr, $right:expr) => {
		$crate::functions::Add::new($left, $right)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::f_const;

	#[test]
	fn test_add() {
		let f = f_add!(f_const!(1), f_const!(2));
		assert_eq!(f.eval(0.0).unwrap(), 3.0);
	}
}
