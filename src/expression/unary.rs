#[macro_use]
mod macros;

use super::{Expression, Function};

/// Unary expression.
///
/// ```bnf
/// u_expr ::= u_op expression
/// u_op ::= "-" | "+"
/// ```
#[derive(Debug, Clone)]
pub enum Unary {
	Neg(Expression),
}

impl Unary {
	pub fn new<S: AsRef<str>, T: Into<Expression>>(op: S, expr: T) -> Option<Self> {
		match op.as_ref() {
			"-" => Some(Self::Neg(expr.into())),
			_ => None,
		}
	}
}

impl std::fmt::Display for Unary {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Unary::Neg(expr) => write!(f, "-{expr}"),
		}
	}
}

impl Function for Unary {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Unary::Neg(expr) => expr.is_x_valid(x),
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Unary::Neg(expr) => Some(-expr.eval(x)?),
		}
	}
}

impl From<Unary> for Expression {
	fn from(val: Unary) -> Self {
		Expression::Unary(val.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::var;

	#[test]
	fn test_neg() {
		let f = neg!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);
	}
}
