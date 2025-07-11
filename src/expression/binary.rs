#[macro_use]
mod macros;

use super::{Expression, Function};

/// Binary expression.
///
/// ```bnf
/// bop_rhs ::= b_op primary
/// b_op ::= "+" | "-" | "*" | "/" | "**"
/// ```
#[derive(Debug, Clone)]
pub enum Binary {
	Add((Expression, Expression)),
	Sub((Expression, Expression)),
	Mul((Expression, Expression)),
	Div((Expression, Expression)),
	Pow((Expression, Expression)),
}

impl Binary {
	pub fn new<S: AsRef<str>, L: Into<Expression>, R: Into<Expression>>(
		op: S,
		lhs: L,
		rhs: R,
	) -> Option<Self> {
		match op.as_ref() {
			"+" => Some(Self::Add((lhs.into(), rhs.into()))),
			"-" => Some(Self::Sub((lhs.into(), rhs.into()))),
			"*" => Some(Self::Mul((lhs.into(), rhs.into()))),
			"/" => Some(Self::Div((lhs.into(), rhs.into()))),
			"**" => Some(Self::Pow((lhs.into(), rhs.into()))),
			_ => None,
		}
	}
}

impl std::fmt::Display for Binary {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Binary::Add((lhs, rhs)) => write!(f, "{lhs} + {rhs}"),
			Binary::Sub((lhs, rhs)) => write!(f, "{lhs} - {rhs}"),
			Binary::Mul((lhs, rhs)) => write!(f, "{lhs} * {rhs}"),
			Binary::Div((lhs, rhs)) => write!(f, "{lhs} / {rhs}"),
			Binary::Pow((lhs, rhs)) => write!(f, "{lhs} ** {rhs}"),
		}
	}
}

impl Function for Binary {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Binary::Add((lhs, rhs)) => lhs.is_x_valid(x) && rhs.is_x_valid(x),
			Binary::Sub((lhs, rhs)) => lhs.is_x_valid(x) && rhs.is_x_valid(x),
			Binary::Mul((lhs, rhs)) => lhs.is_x_valid(x) && rhs.is_x_valid(x),
			Binary::Div((lhs, rhs)) => lhs.is_x_valid(x) && rhs.eval(x).is_some_and(|v| v != 0.0),
			Binary::Pow((lhs, rhs)) => {
				let Some(lhs) = lhs.eval(x) else {
					return false;
				};
				let Some(rhs) = rhs.eval(x) else {
					return false;
				};

				// 0.0 cannot be raised to a negative power
				if lhs == 0.0 && rhs < 0.0 {
					return false;
				}

				// we consider complex number invalid
				!lhs.powf(rhs).is_nan()
			}
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Binary::Add((lhs, rhs)) => Some(lhs.eval(x)? + rhs.eval(x)?),
			Binary::Sub((lhs, rhs)) => Some(lhs.eval(x)? - rhs.eval(x)?),
			Binary::Mul((lhs, rhs)) => Some(lhs.eval(x)? * rhs.eval(x)?),
			Binary::Div((lhs, rhs)) => {
				if self.is_x_valid(x) {
					Some(lhs.eval(x).unwrap() / rhs.eval(x).unwrap())
				} else {
					None
				}
			}
			Binary::Pow((lhs, rhs)) => {
				if self.is_x_valid(x) {
					Some(lhs.eval(x).unwrap().powf(rhs.eval(x).unwrap()))
				} else {
					None
				}
			}
		}
	}
}

impl From<Binary> for Expression {
	fn from(val: Binary) -> Self {
		Expression::Binary(val.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{num, var};

	#[test]
	fn test_add() {
		let f = add!(var!(), num!(1.0));
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(1.0).unwrap(), 2.0);
	}

	#[test]
	fn test_sub() {
		let f = sub!(var!(), num!(1.0));
		assert_eq!(f.eval(0.0).unwrap(), -1.0);
		assert_eq!(f.eval(1.0).unwrap(), 0.0);
	}

	#[test]
	fn test_mul() {
		let f = mul!(var!(), num!(1.0));
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_div() {
		let f = div!(num!(1.0), var!());
		assert_eq!(f.eval(2.0).unwrap(), 0.5);
		assert!(f.eval(0.0).is_none());
	}

	#[test]
	fn test_pow() {
		let f = pow!(var!(), num!(2.0));
		assert_eq!(f.eval(-9.0).unwrap(), 81.0);
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(9.0).unwrap(), 81.0);

		let f = pow!(var!(), num!(0.0));
		assert_eq!(f.eval(-9.0).unwrap(), 1.0);
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(9.0).unwrap(), 1.0);

		let f = pow!(var!(), num!(0.5));
		assert!(f.eval(-9.0).is_none());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(9.0).unwrap(), 3.0);

		let f = pow!(var!(), num!(-2.0));
		assert_eq!(f.eval(-9.0).unwrap(), (-9.0f32).powf(-2.0));
		assert!(f.eval(0.0).is_none());
		assert_eq!(f.eval(09.0).unwrap(), (9.0f32).powf(-2.0));
	}
}
