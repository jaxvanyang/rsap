use std::rc::Rc;

use super::{function::Function, utils::is_zero};

pub struct Divide {
	left: Rc<dyn Function>,
	right: Rc<dyn Function>,
}

impl Divide {
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

impl std::fmt::Display for Divide {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} / {}", self.left, self.right)
	}
}

impl Function for Divide {
	fn is_x_valid(&self, x: f32) -> bool {
		if let Some(right) = self.right.eval(x) {
			self.left.is_x_valid(x) && !is_zero(right)
		} else {
			false
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		if self.is_x_valid(x) {
			Some(self.left.eval(x)? / self.right.eval(x)?)
		} else {
			None
		}
	}
}

impl From<Divide> for Rc<dyn Function> {
	fn from(val: Divide) -> Self {
		Rc::new(val)
	}
}

/// Create a function to divide two inputs.
#[macro_export]
macro_rules! f_div {
	($left:expr, $right:expr) => {
		$crate::functions::Divide::new($left, $right)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::f_const;

	#[test]
	fn test_div() {
		let f = f_div!(f_const!(1), f_const!(2));
		assert_eq!(f.eval(0.0).unwrap(), 0.5);
	}

	#[test]
	fn test_divide_by_zero() {
		let f = f_div!(f_const!(1), f_const!(0));
		assert!(f.eval(0.0).is_none())
	}
}
