use std::rc::Rc;

use super::function::Function;

/// The input x.
#[derive(Debug, Clone, Copy, Default)]
pub struct Variable;

impl Variable {
	#[must_use]
	pub fn new() -> Self {
		Self
	}
}

impl std::fmt::Display for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "x")
	}
}

impl Function for Variable {
	fn eval(&self, x: f32) -> Option<f32> {
		Some(x)
	}
}

impl From<Variable> for Rc<dyn Function> {
	fn from(val: Variable) -> Self {
		Rc::new(val)
	}
}

/// Create the input variable x.
#[macro_export]
macro_rules! f_var {
	() => {
		$crate::functions::Variable::new()
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_variable() {
		let f = f_var!();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}
}
