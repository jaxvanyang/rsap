use super::{Expression, Function};

#[derive(Debug, Clone, Default)]
pub struct Variable;

impl Variable {
	pub fn new() -> Self {
		Self::default()
	}
}

#[macro_export]
macro_rules! var {
	() => {
		$crate::expression::Variable::new()
	};
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

impl Into<Expression> for Variable {
	fn into(self) -> Expression {
		Expression::Variable(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_variable() {
		let f = var!();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}
}
