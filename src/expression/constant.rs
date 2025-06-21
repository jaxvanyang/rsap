use super::{Expression, Function};
use std::f32::consts::{E, PI};

/// Constant expression.
///
/// ```bnf
/// constant ::= "e" | "pi"
/// ```
#[derive(Debug, Clone)]
pub enum Constant {
	E,
	PI,
}

impl Constant {
	pub fn new<S: AsRef<str>>(s: S) -> Option<Self> {
		Some(match s.as_ref() {
			"e" => Self::E,
			"pi" => Self::PI,
			_ => return None,
		})
	}
}

#[macro_export]
macro_rules! constant {
	($s:expr) => {
		$crate::expression::Constant::new($s)
	};
}

impl std::fmt::Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::E => "e",
				Self::PI => "pi",
			}
		)
	}
}

impl Function for Constant {
	fn eval(&self, _x: f32) -> Option<f32> {
		Some(match self {
			Self::E => E,
			Self::PI => PI,
		})
	}
}

impl From<Constant> for Expression {
	fn from(val: Constant) -> Self {
		Expression::Constant(val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constant() {
		let e = constant!("e").unwrap();
		assert_eq!(e.eval(0.0).unwrap(), E);
		assert_eq!(e.eval(1.0).unwrap(), E);

		let pi = constant!("pi").unwrap();
		assert_eq!(pi.eval(0.0).unwrap(), PI);
		assert_eq!(pi.eval(1.0).unwrap(), PI);
	}
}
