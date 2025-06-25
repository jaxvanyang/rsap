use super::{Expression, Function};
use crate::math::factorial;

/// Factorial expression.
///
/// ```bnf
/// factorial ::= (digit)+ "!"
/// digit ::= "0"..."9"
/// ```
#[derive(Debug, Clone)]
pub struct Factorial(u32);

impl Factorial {
	#[must_use]
	pub fn new(n: u32) -> Self {
		Self(n)
	}
}

#[macro_export]
macro_rules! factorial {
	($n:expr) => {
		$crate::expression::Factorial::new($n)
	};
}

impl std::fmt::Display for Factorial {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}!", self.0)
	}
}

impl Function for Factorial {
	fn eval(&self, _x: f32) -> Option<f32> {
		Some(factorial(self.0) as f32)
	}
}

impl From<Factorial> for Expression {
	fn from(val: Factorial) -> Self {
		Expression::Factorial(val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_factorial() {
		let f = factorial!(5);
		assert_eq!(f.eval(0.0).unwrap(), 120.0);
		assert_eq!(f.eval(1.0).unwrap(), 120.0);
	}
}
