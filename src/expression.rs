//! Function expression.

pub mod binary;
pub mod constant;
pub mod factorial;
pub mod func;
pub mod func2;
pub mod lexer;
pub mod number;
pub mod parenthesis;
pub mod parser;
pub mod traits;
pub mod unary;
pub mod variable;

pub use binary::*;
pub use constant::*;
pub use factorial::*;
pub use func::Func;
pub use func2::Func2;
pub use lexer::*;
pub use number::*;
pub use parenthesis::*;
pub use parser::*;
pub use traits::*;
pub use unary::*;
pub use variable::*;

/// Top-level expression
#[derive(Debug, Clone)]
pub enum Expression {
	Number(Number),
	Factorial(Factorial),
	Constant(Constant),
	Variable(Variable),
	Unary(Box<Unary>),
	Parenthesis(Box<Parenthesis>),
	Binary(Box<Binary>),
	Func(Box<Func>),
	Func2(Box<Func2>),
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Number(e) => e.fmt(f),
			Expression::Factorial(e) => e.fmt(f),
			Expression::Variable(e) => e.fmt(f),
			Expression::Constant(e) => e.fmt(f),
			Expression::Unary(e) => e.fmt(f),
			Expression::Parenthesis(e) => e.fmt(f),
			Expression::Binary(e) => e.fmt(f),
			Expression::Func(e) => e.fmt(f),
			Expression::Func2(e) => e.fmt(f),
		}
	}
}

impl Function for Expression {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Expression::Unary(e) => e.is_x_valid(x),
			Expression::Parenthesis(e) => e.is_x_valid(x),
			Expression::Binary(e) => e.is_x_valid(x),
			Expression::Func(e) => e.is_x_valid(x),
			Expression::Func2(e) => e.is_x_valid(x),
			_ => true,
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Expression::Number(e) => e.eval(x),
			Expression::Factorial(e) => e.eval(x),
			Expression::Variable(e) => e.eval(x),
			Expression::Constant(e) => e.eval(x),
			Expression::Unary(e) => e.eval(x),
			Expression::Parenthesis(e) => e.eval(x),
			Expression::Binary(e) => e.eval(x),
			Expression::Func(e) => e.eval(x),
			Expression::Func2(e) => e.eval(x),
		}
	}
}
