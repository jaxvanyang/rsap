use super::{Expression, Function};

/// Number expression.
///
/// ```bnf
/// number ::= (digit)+ ["." (digit)+]
/// digit ::= "0"..."9"
/// ```
#[derive(Debug, Clone)]
pub struct Number(f32);

impl Number {
	#[must_use]
	pub fn new(n: f32) -> Self {
		Self(n)
	}
}

#[macro_export]
macro_rules! num {
	($n:expr) => {
		$crate::expression::Number::new($n)
	};
}

impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Function for Number {
	fn eval(&self, _x: f32) -> Option<f32> {
		Some(self.0)
	}
}

impl From<Number> for Expression {
	fn from(val: Number) -> Self {
		Expression::Number(val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_num() {
		let f = num!(5.0);
		assert_eq!(f.eval(0.0).unwrap(), 5.0);
		assert_eq!(f.eval(1.0).unwrap(), 5.0);
	}
}
