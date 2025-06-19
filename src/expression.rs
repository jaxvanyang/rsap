//! Function expression.

pub mod binary;
pub mod lexer;
pub mod number;
pub mod parenthesis;
pub mod parse;
pub mod traits;
pub mod unary;
pub mod variable;

pub use binary::*;
pub use lexer::*;
pub use number::*;
pub use parenthesis::*;
pub use parse::*;
pub use traits::*;
pub use unary::*;
pub use variable::*;

/// Top-level expression
///
/// ```bnf
/// expression ::= primary (bop_rhs)*
/// primary ::= number | variable | u_expr | p_expr
/// ```
#[derive(Debug, Clone)]
pub enum Expression {
	Number(Number),
	Variable(Variable),
	Unary(Box<Unary>),
	Parenthesis(Box<Parenthesis>),
	Binary(Box<Binary>),
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Number(number) => number.fmt(f),
			Expression::Variable(variable) => variable.fmt(f),
			Expression::Unary(unary) => unary.fmt(f),
			Expression::Parenthesis(parenthesis) => parenthesis.fmt(f),
			Expression::Binary(binary) => binary.fmt(f),
		}
	}
}

impl Function for Expression {
	fn is_x_valid(&self, x: f32) -> bool {
		match self {
			Expression::Number(number) => number.is_x_valid(x),
			Expression::Variable(variable) => variable.is_x_valid(x),
			Expression::Unary(unary) => unary.is_x_valid(x),
			Expression::Parenthesis(parenthesis) => parenthesis.is_x_valid(x),
			Expression::Binary(binary) => binary.is_x_valid(x),
		}
	}

	fn eval(&self, x: f32) -> Option<f32> {
		match self {
			Expression::Number(number) => number.eval(x),
			Expression::Variable(variable) => variable.eval(x),
			Expression::Unary(unary) => unary.eval(x),
			Expression::Parenthesis(parenthesis) => parenthesis.eval(x),
			Expression::Binary(binary) => binary.eval(x),
		}
	}
}
