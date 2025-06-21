use crate::{binary_expr, constant, func, neg, num, paren, var};

use super::{
	lexer::{Lexer, Token},
	Expression, FUNCTION_NAMES,
};

#[derive(Debug, Clone)]
pub struct Parser {
	current: Token,
	lexer: Lexer,
}

impl Parser {
	pub fn new<T: ToString>(expr: T) -> Self {
		let mut out = Self {
			current: Token::Eof,
			lexer: Lexer::new(expr),
		};
		out.get_next();

		out
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
	fn parse_variable(&mut self) -> Expression {
		// we know this is a "x"
		let x = var!().into();

		self.get_next();

		x
	}

	/// Parse constant expression.
	///
	/// ```bnf
	/// constant ::= "e" | "pi"
	/// ```
	fn parse_constant(&mut self) -> Expression {
		let Token::Identifier(c) = self.current.clone() else {
			unreachable!()
		};

		self.get_next();

		constant!(c).unwrap().into()
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
			}
			anyhow::bail!("only - allowed");
		}
		unreachable!()
	}

	/// Parse parenthesis expression.
	///
	/// ```bnf
	/// p_expr ::= "(" sub_expr ")"
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

	/// Parse function expression.
	///
	/// ```bnf
	/// f_expr ::= f_name "(" sub_expr ")"
	/// f_name ::= "sin"
	/// ```
	fn parse_function(&mut self) -> anyhow::Result<Expression> {
		let Token::Identifier(f_name) = self.current.clone() else {
			anyhow::bail!("expected a function name, but found: {:?}", self.current);
		};

		if !FUNCTION_NAMES.contains(&f_name.as_str()) {
			anyhow::bail!(
				"expected a valid function name, but found: {:?}",
				self.current
			);
		}

		// eat f_name
		self.get_next();

		if !self.current.is_open_parenthesis() {
			anyhow::bail!("expected `(`, but found: {:?}", self.current);
		}

		// eat "("
		self.get_next();

		let expr = self.parse_sub()?;

		if !self.current.is_close_parenthesis() {
			anyhow::bail!("expected `)`, but found: {:?}", self.current);
		}

		// eat ")"
		self.get_next();

		Ok(func!(f_name, expr).unwrap().into())
	}

	/// Parse primary expression.
	///
	/// ```bnf
	/// primary ::= number | variable | constant | u_expr | p_expr | f_expr
	/// ```
	fn parse_primary(&mut self) -> anyhow::Result<Expression> {
		match &self.current {
			Token::Number(_) => Ok(self.parse_number()),
			Token::Identifier(id) => match id.as_str() {
				"x" => Ok(self.parse_variable()),
				"e" | "pi" => Ok(self.parse_constant()),
				_ => self.parse_function(),
			},
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

#[inline]
pub fn parse<T: ToString>(expr: T) -> anyhow::Result<Expression> {
	Parser::new(expr).parse()
}

#[cfg(test)]
mod tests {
	use super::{super::Function, *};

	#[test]
	fn test_parse() {
		let f = parse("-x + 1 * 2").unwrap();
		assert_eq!(f.to_string(), "-x + 1 * 2");
		assert_eq!(f.eval(0.0).unwrap(), 2.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_parser() {
		let mut parser = Parser::new(" -x + 1 * 2");
		assert_eq!(parser.current, Token::Operator("-".to_string()));
		parser.get_next();
		assert_eq!(parser.current, Token::Identifier("x".to_string()));
		parser.get_next();
		assert_eq!(parser.current, Token::Operator("+".to_string()));
	}

	#[test]
	fn test_parse_primary() {
		let f = parse("1").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 1.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let f = parse("x").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let f = parse("-x").unwrap();
		assert_eq!(f.eval(0.0).unwrap(), 0.0);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);
	}

	#[test]
	fn test_parse_number() {
		let expr = "1";
		assert_eq!(parse(expr).unwrap().to_string(), expr);

		let expr = "1.0";
		assert_eq!(parse(expr).unwrap().to_string(), "1");

		let expr = "0.3";
		assert_eq!(parse(expr).unwrap().to_string(), expr);

		assert!(parse("1.").is_err());
	}

	#[test]
	fn test_parse_variable() {
		let expr = "x";
		assert_eq!(parse(expr).unwrap().to_string(), expr);

		assert!(parse("y").is_err());
	}

	#[test]
	fn test_parse_constant() {
		let expr = "e";
		assert_eq!(parse(expr).unwrap().to_string(), expr);

		let expr = "pi";
		assert_eq!(parse(expr).unwrap().to_string(), expr);

		assert!(parse("C").is_err());
	}

	#[test]
	fn test_parse_neg() {
		let expr = "-x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), -1.0);

		let expr = "--x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_parse_binary() {
		let expr = "x + x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 2.0);

		let expr = "x - x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 0.0);

		let expr = "x * x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let expr = "x / x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);

		let expr = "x ** x";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 1.0);
	}

	#[test]
	fn test_parse_paren() {
		let expr = "(x + 1) * 2";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(1.0).unwrap(), 4.0);
	}

	#[test]
	fn test_parse_func() {
		let expr = "sin(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 0.0);

		let expr = "cos(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 1.0);

		let expr = "tan(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 0.0);

		let expr = "cot(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert!(f.eval(0.0).is_none());

		let expr = "sec(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 1.0);

		let expr = "csc(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert!(f.eval(0.0).is_none());

		let expr = "arcsin(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 0.0);

		let expr = "arccos(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), std::f32::consts::FRAC_PI_2);

		let expr = "arctan(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), 0.0);

		let expr = "arccot(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert_eq!(f.eval(0.0).unwrap(), std::f32::consts::FRAC_PI_2);

		let expr = "ln(x)";
		let f = parse(expr).unwrap();
		assert_eq!(f.to_string(), expr);
		assert!(f.eval(0.0).is_none());
	}
}
