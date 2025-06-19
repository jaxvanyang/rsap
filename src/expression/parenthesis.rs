use super::{Expression, Function};

/// Parenthesis expression.
///
/// ```bnf
/// p_expr ::= "(" expression ")"
/// ```
#[derive(Debug, Clone)]
pub struct Parenthesis(Expression);

impl Parenthesis {
	pub fn new<T: Into<Expression>>(expr: T) -> Self {
		Self(expr.into())
	}
}

impl std::fmt::Display for Parenthesis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({})", self.0)
	}
}

impl Function for Parenthesis {
	fn is_x_valid(&self, x: f32) -> bool {
		self.0.is_x_valid(x)
	}

	fn eval(&self, x: f32) -> Option<f32> {
		self.0.eval(x)
	}
}

impl From<Parenthesis> for Expression {
	fn from(val: Parenthesis) -> Self {
		Expression::Parenthesis(val.into())
	}
}

#[macro_export]
macro_rules! paren {
	($e:expr) => {
		$crate::expression::Parenthesis::new($e)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{add, mul, num};

	#[test]
	fn test_paren() {
		// (1 + 2) * 3
		let f = mul!(paren!(add!(num!(1.0), num!(2.0))), num!(3.0));
		assert_eq!(f.to_string(), "(1 + 2) * 3");
		assert_eq!(f.eval(0.0).unwrap(), 9.0);
		assert_eq!(f.eval(1.0).unwrap(), 9.0);
	}
}
