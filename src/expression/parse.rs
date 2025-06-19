use crate::{binary_expr, neg, num, paren, var};

use super::{
	lexer::{Lexer, Token},
	Expression,
};

#[derive(Debug, Clone)]
pub struct Parser {
	current: Token,
	lexer: Lexer,
}

impl Parser {
	pub fn new<T: ToString>(expr: T) -> Self {
		let mut lexer = Lexer::new(expr);

		Self {
			current: lexer.next().unwrap_or(Token::Eof),
			lexer,
		}
	}

	/// Update current token to the next non-whitespace token by lexer.
	///
	/// Set current token to `Eof` if no more token.
	fn get_next(&mut self) {
		if let Some(token) = self.lexer.next() {
			self.current = token;
		} else {
			self.current = Token::Eof;
		}

		if self.current.is_whitespace() {
			self.get_next();
		}
	}
	/// Parse umber expression.
	///
	/// ```bnf
	/// number ::= (digit)+ ["." (digit)+]
	/// digit ::= "0"..."9"
	/// ```
	fn parse_number(&mut self) -> Expression {
		let expr = if let Token::Number(n) = self.current {
			num!(n).into()
		} else {
			unreachable!()
		};

		self.get_next();

		expr
	}

	/// Parse variable expression, i.e., x.
	///
	/// ```bnf
	/// variable ::= "x"
	/// ```
	fn parse_variable(&mut self) -> anyhow::Result<Expression> {
		let expr = if let Token::Identifier(id) = &self.current {
			if id == "x" {
				var!().into()
			} else {
				anyhow::bail!("only x allowed");
			}
		} else {
			unreachable!()
		};

		self.get_next();

		Ok(expr)
	}

	/// Parse unary expression.
	///
	/// ```bnf
	/// u_expr ::= u_op expression
	/// u_op ::= "-" | "+"
	/// ```
	fn parse_unary(&mut self) -> anyhow::Result<Expression> {
		if let Token::Operator(op) = &self.current {
			if op == "-" {
				self.get_next();
				return Ok(neg!(self.parse_primary()?).into());
			} else {
				anyhow::bail!("only - allowed");
			}
		}
		unreachable!()
	}

	/// Parse parenthesis expression.
	///
	/// ```bnf
	/// p_expr ::= "(" expression ")"
	/// ```
	fn parse_parenthesis(&mut self) -> anyhow::Result<Expression> {
		assert!(self.current.is_open_parenthesis());

		// eat "("
		self.get_next();

		let expr = self.parse_sub()?;

		if !self.current.is_close_parenthesis() {
			anyhow::bail!("expected `)`, but found: {:?}", self.current);
		}

		self.get_next();

		Ok(paren!(expr).into())
	}

	/// Parse primary expression.
	///
	/// ```bnf
	/// primary ::= number | variable | u_expr | p_expr
	/// ```
	fn parse_primary(&mut self) -> anyhow::Result<Expression> {
		match self.current {
			Token::Number(_) => Ok(self.parse_number()),
			Token::Identifier(_) => self.parse_variable(),
			Token::Operator(_) => self.parse_unary(),
			Token::OpenParenthesis => self.parse_parenthesis(),
			_ => anyhow::bail!("not expected: {:?}", self.current),
		}
	}

	/// Parse sub-expression (including empty) with equal or highter precedence than `lhs`.
	///
	/// ```bnf
	/// b_subexpr ::= (bop_rhs)*
	/// bop_rhs ::= b_op primary
	/// b_op ::= "+" | "-" | "*" | "/"
	/// ```
	fn parse_op_rhs(
		&mut self,
		mut lhs: Expression,
		lhs_precedence: u8,
	) -> anyhow::Result<Expression> {
		loop {
			match &self.current {
				Token::Operator(op) => {
					let op = op.clone();
					let token_precedence = self.current.precedence().unwrap();

					// Current token has lower precedence means that we are done for the sub-expression.
					if token_precedence < lhs_precedence {
						return Ok(lhs);
					}

					self.get_next();

					// same or higher precedence means that we should consider the next op
					let mut rhs = self.parse_primary()?;

					// cannot merge, so consume all sub-expressions with higher precedence first
					if self
						.current
						.precedence()
						.is_some_and(|p| token_precedence < p)
					{
						rhs = self.parse_op_rhs(rhs, token_precedence + 1)?;
					}

					lhs = binary_expr!(&op, lhs, rhs).unwrap().into();
				}
				_ => return Ok(lhs),
			}
		}
	}

	/// Parse sub-expression.
	///
	/// ```bnf
	/// sub_expr ::= primary b_subexpr
	/// ```
	pub fn parse_sub(&mut self) -> anyhow::Result<Expression> {
		let lhs = self.parse_primary()?;

		self.parse_op_rhs(lhs, 0)
	}

	/// Parse top-level expression.
	///
	/// ```bnf
	/// expression ::= sub_expr eof
	/// ```
	pub fn parse(&mut self) -> anyhow::Result<Expression> {
		let result = self.parse_sub();

		if !self.current.is_eof() {
			anyhow::bail!("expected EOF, but found: {:?}", self.current);
		}

		result
	}
}

#[macro_export]
macro_rules! parse {
	($e:expr) => {
		$crate::expression::Parser::new($e).parse()
	};
}

#[cfg(test)]
mod tests {
	use super::{super::Function, *};

	#[test]
	fn test_parse() {
		let f = parse!("-x + 1 * 2").unwrap();
		assert_eq!(f.to_string(), "-x + 1 * 2");
		assert_eq!(f.eval(0.0).unwrap(), 2.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_parser() {
		let parser = Parser::new("-x + 1 * 2");
		assert_ne!(parser.current, Token::Eof);
	}

	#[test]
	fn test_parse_primary() {
		let f = parse!("1").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let f = parse!("x").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let f = parse!("-x").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);
	}

	#[test]
	fn test_parse_number() {
		let expr = "1";
		assert_eq!(parse!(expr).unwrap().to_string(), expr);

		let expr = "1.0";
		assert_eq!(parse!(expr).unwrap().to_string(), "1");

		let expr = "0.3";
		assert_eq!(parse!(expr).unwrap().to_string(), expr);

		assert!(parse!("1.").is_err());
	}

	#[test]
	fn test_parse_variable() {
		let expr = "x";
		assert_eq!(parse!(expr).unwrap().to_string(), expr);

		assert!(parse!("y").is_err());
	}

	#[test]
	fn test_parse_neg() {
		let expr = "-x";
		let f = parse!(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);

		let expr = "--x";
		let f = parse!(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_parse_binary() {
		let expr = "x + x";
		let f = parse!(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 2.0);
	}

	#[test]
	fn test_parse_paren() {
		let expr = "(x + 1) * 2";
		let f = parse!(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 4.0);
	}
}
