use super::{Expression, Function};

/// Variable expression, i.e., x.
///
/// ```bnf
/// variable ::= "x"
/// ```
#[derive(Debug, Clone, Default)]
pub struct Variable;

impl Variable {
	#[must_use]
	pub fn new() -> Self {
		Self
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

impl From<Variable> for Expression {
	fn from(val: Variable) -> Self {
		Expression::Variable(val)
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
