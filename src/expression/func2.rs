//! Two-argument functions.

#[macro_use]
mod macros;

use crate::utils::is_equal;

use super::{Expression, Function};

pub const FUNCTION_NAMES: [&str; 1] = ["log"];

/// Function expression.
#[derive(Debug, Clone)]
pub enum Func2 {
	Log((Expression, Expression)),
}

impl Func2 {
	pub fn new<S: AsRef<str>, L: Into<Expression>, R: Into<Expression>>(
		f_name: S,
		lhs: L,
		rhs: R,
	) -> Option<Self> {
		match f_name.as_ref() {
			"log" => Some(Self::Log((lhs.into(), rhs.into()))),
			_ => None,
		}
	}
}

impl std::fmt::Display for Func2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Func2::Log((lhs, rhs)) => write!(f, "log({lhs}, {rhs})"),
		}
	}
}

impl Function for Func2 {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Func2::Log((lhs, rhs)) => lhs.eval(x).is_some_and(|a| {
				a > 0.0 && !is_equal(a, 1.0) && rhs.eval(x).is_some_and(|x| x > 0.0)
			}),
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Func2::Log((lhs, rhs)) => self
				.is_x_valid(x)
				.then_some(rhs.eval(x).unwrap().log(lhs.eval(x).unwrap())),
		}
	}
}

impl From<Func2> for Expression {
	fn from(val: Func2) -> Self {
		Expression::Func2(val.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{num, var};

	#[test]
	fn test_log() {
		let f = log!(num!(10.0), var!());
		assert_eq!(f.eval(1.0).unwrap(), 0.0);
		assert_eq!(f.eval(10.0).unwrap(), 1.0);
	}
}
