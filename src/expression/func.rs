#[macro_use]
mod macros;

use super::{Expression, Function};

pub const FUNCTION_NAMES: [&str; 10] = [
	"sin", "cos", "tan", "cot", "sec", "csc", "arcsin", "arccos", "arctan", "arccot",
];

/// Function expression.
#[derive(Debug, Clone)]
pub enum Func {
	Sin(Expression),
	Cos(Expression),
	Tan(Expression),
	Cot(Expression),
	Sec(Expression),
	Csc(Expression),
	Arcsin(Expression),
	Arccos(Expression),
	Arctan(Expression),
	Arccot(Expression),
}

impl Func {
	pub fn new<S: AsRef<str>, T: Into<Expression>>(f_name: S, expr: T) -> Option<Self> {
		match f_name.as_ref() {
			"sin" => Some(Self::Sin(expr.into())),
			"cos" => Some(Self::Cos(expr.into())),
			"tan" => Some(Self::Tan(expr.into())),
			"cot" => Some(Self::Cot(expr.into())),
			"sec" => Some(Self::Sec(expr.into())),
			"csc" => Some(Self::Csc(expr.into())),
			"arcsin" => Some(Self::Arcsin(expr.into())),
			"arccos" => Some(Self::Arccos(expr.into())),
			"arctan" => Some(Self::Arctan(expr.into())),
			"arccot" => Some(Self::Arccot(expr.into())),
			_ => None,
		}
	}
}

impl std::fmt::Display for Func {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Func::Sin(expr) => write!(f, "sin({expr})"),
			Func::Cos(expr) => write!(f, "cos({expr})"),
			Func::Tan(expr) => write!(f, "tan({expr})"),
			Func::Cot(expr) => write!(f, "cot({expr})"),
			Func::Sec(expr) => write!(f, "sec({expr})"),
			Func::Csc(expr) => write!(f, "csc({expr})"),
			Func::Arcsin(expr) => write!(f, "arcsin({expr})"),
			Func::Arccos(expr) => write!(f, "arccos({expr})"),
			Func::Arctan(expr) => write!(f, "arctan({expr})"),
			Func::Arccot(expr) => write!(f, "arccot({expr})"),
		}
	}
}

impl Function for Func {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Func::Sin(expr) => expr.is_x_valid(x),
			Func::Cos(expr) => expr.is_x_valid(x),
			Func::Tan(expr) => expr.eval(x).is_some_and(|val| val.tan().is_finite()),
			Func::Cot(expr) => expr
				.eval(x)
				.is_some_and(|val| (1.0 / val.tan()).is_finite()),
			Func::Sec(expr) => expr.eval(x).is_some_and(|val| val.cos() != 0.0),
			Func::Csc(expr) => expr.eval(x).is_some_and(|val| val.sin() != 0.0),
			Func::Arcsin(expr) => expr.eval(x).is_some_and(|val| (-1.0..=1.0).contains(&val)),
			Func::Arccos(expr) => expr.eval(x).is_some_and(|val| (-1.0..=1.0).contains(&val)),
			Func::Arctan(expr) => expr.is_x_valid(x),
			Func::Arccot(expr) => expr.is_x_valid(x),
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Func::Sin(expr) => Some(expr.eval(x)?.sin()),
			Func::Cos(expr) => Some(expr.eval(x)?.cos()),
			Func::Tan(expr) => self.is_x_valid(x).then_some(expr.eval(x)?.tan()),
			Func::Cot(expr) => self.is_x_valid(x).then_some(1.0 / expr.eval(x)?.tan()),
			Func::Sec(expr) => self.is_x_valid(x).then_some(1.0 / expr.eval(x)?.cos()),
			Func::Csc(expr) => self.is_x_valid(x).then_some(1.0 / expr.eval(x)?.sin()),
			Func::Arcsin(expr) => self.is_x_valid(x).then_some(expr.eval(x)?.asin()),
			Func::Arccos(expr) => self.is_x_valid(x).then_some(expr.eval(x)?.acos()),
			Func::Arctan(expr) => expr.eval(x).map(f32::atan),
			Func::Arccot(expr) => expr
				.eval(x).map(|val| std::f32::consts::FRAC_PI_2 - val.atan()),
		}
	}
}

impl From<Func> for Expression {
	fn from(val: Func) -> Self {
		Expression::Func(val.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::var;

	#[test]
	fn test_sin() {
		let f = sin!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0f32.sin());
	}

	#[test]
	fn test_cos() {
		let f = cos!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0f32.cos());
	}

	#[test]
	fn test_tan() {
		let f = tan!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0f32.tan());
	}

	#[test]
	fn test_cot() {
		let f = cot!(var!());
		assert!(f.eval(0.0).is_none());
		assert_eq!(f.eval(1.0).unwrap(), 1.0 / 1.0f32.tan());
	}

	#[test]
	fn test_sec() {
		let f = sec!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0 / 1.0f32.cos());
	}

	#[test]
	fn test_csc() {
		let f = csc!(var!());
		assert!(f.eval(0.0).is_none());
		assert_eq!(f.eval(1.0).unwrap(), 1.0 / 1.0f32.sin());
	}

	#[test]
	fn test_arcsin() {
		let f = arcsin!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0f32.asin());
	}

	#[test]
	fn test_arccos() {
		let f = arccos!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0f32.acos());
		assert_eq!(f.eval(1.0).unwrap(), 0.0);
	}

	#[test]
	fn test_arctan() {
		let f = arctan!(var!());
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0f32.atan());
	}

	#[test]
	fn test_arccot() {
		let f = arccot!(var!());
		assert_eq!(f.eval(0.0).unwrap(), std::f32::consts::FRAC_PI_2);
		assert_eq!(
			f.eval(1.0).unwrap(),
			std::f32::consts::FRAC_PI_2 - 1.0f32.atan()
		);
	}
}
