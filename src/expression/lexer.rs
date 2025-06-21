// NOTE: longer first
pub const OPERATORS: [&str; 5] = ["**", "+", "-", "*", "/"];

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	WhiteSpace(char),
	Number(f32),
	/// ```bnf
	/// id ::= ("a"..."z" | "A"..."Z")+
	/// ```
	Identifier(String),
	/// ```bnf
	/// op ::= "+" | "-" | "*" | "/"
	Operator(String),
	OpenParenthesis,
	CloseParenthesis,
	Eof,
	/// Invalid sub-string or other character.
	Other(String),
}

impl Token {
	#[inline]
	#[must_use]
	pub fn is_whitespace(&self) -> bool {
		if let Self::WhiteSpace(_) = self {
			true
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_eof(&self) -> bool {
		if let Self::Eof = self {
			true
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_operator(&self) -> bool {
		if let Self::Operator(_) = self {
			true
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_open_parenthesis(&self) -> bool {
		if let Self::OpenParenthesis = self {
			true
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_close_parenthesis(&self) -> bool {
		if let Self::CloseParenthesis = self {
			true
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_comma(&self) -> bool {
		if let Self::Other(s) = self {
			s == ","
		} else {
			false
		}
	}

	/// Token precedence.
	///
	/// Return `Some` if self is an operator.
	#[inline]
	#[must_use]
	pub fn precedence(&self) -> Option<u8> {
		if let Token::Operator(op) = self {
			Some(match op.as_str() {
				"+" | "-" => 10,
				"*" | "/" => 20,
				"**" => 30,
				_ => unreachable!(),
			})
		} else {
			None
		}
	}
}

#[derive(Debug, Clone)]
pub struct Lexer {
	chars: Vec<char>,
	/// current index
	i: usize,
}

impl Lexer {
	pub fn new<T: ToString>(expr: T) -> Self {
		Self {
			chars: expr.to_string().chars().collect(),
			i: 0,
		}
	}

	/// Helper function for `Iterator::next`.
	#[inline]
	fn get_token(&self, i: usize) -> Option<(Token, usize)> {
		let chars = &self.chars;

		// only consider run-out as EOF
		if i >= chars.len() {
			return None;
		}

		// check white space
		if chars[i].is_whitespace() {
			return Some((Token::WhiteSpace(chars[i]), i + 1));
		}

		// check number
		if chars[i].is_ascii_digit() {
			let mut j = i + 1;
			while chars.get(j).is_some_and(char::is_ascii_digit) {
				j += 1;
			}

			if chars.get(j).is_some_and(|c| *c == '.') {
				j += 1;

				if !chars.get(j).is_some_and(char::is_ascii_digit) {
					return Some((Token::Other(chars[i..j].iter().collect()), j));
				}

				while chars.get(j).is_some_and(char::is_ascii_digit) {
					j += 1;
				}
			}

			let c: String = chars[i..j].iter().collect();
			let c: f32 = c.parse().unwrap();

			return Some((Token::Number(c), j));
		}

		// check identifier
		if chars[i].is_ascii_alphabetic() {
			let mut j = i + 1;
			while chars.get(j).is_some_and(char::is_ascii_alphabetic) {
				j += 1;
			}

			return Some((Token::Identifier(chars[i..j].iter().collect()), j));
		}

		// check operator
		for op in OPERATORS {
			// max length of operators is 2
			let s: String = chars[i..(chars.len().min(i + 2))].iter().collect();
			if s.starts_with(op) {
				return Some((Token::Operator(op.to_string()), i + op.len()));
			}
		}

		// check parenthesis
		if chars[i] == '(' {
			return Some((Token::OpenParenthesis, i + 1));
		}
		if chars[i] == ')' {
			return Some((Token::CloseParenthesis, i + 1));
		}

		Some((Token::Other(chars[i..=i].iter().collect()), i + 1))
	}
}

impl std::fmt::Display for Lexer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"expression: {:?}, position: {}",
			self.chars.iter().collect::<String>(),
			self.i
		)
	}
}

impl Iterator for Lexer {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some((token, i)) = self.get_token(self.i) {
			self.i = i;
			Some(token)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize() {
		let tokens: Vec<_> = Lexer::new("-x + (1 - 2) * 3 / 4").collect();
		assert_eq!(
			tokens,
			[
				Token::Operator("-".to_string()),
				Token::Identifier("x".to_string()),
				Token::WhiteSpace(' '),
				Token::Operator("+".to_string()),
				Token::WhiteSpace(' '),
				Token::OpenParenthesis,
				Token::Number(1.0),
				Token::WhiteSpace(' '),
				Token::Operator("-".to_string()),
				Token::WhiteSpace(' '),
				Token::Number(2.0),
				Token::CloseParenthesis,
				Token::WhiteSpace(' '),
				Token::Operator("*".to_string()),
				Token::WhiteSpace(' '),
				Token::Number(3.0),
				Token::WhiteSpace(' '),
				Token::Operator("/".to_string()),
				Token::WhiteSpace(' '),
				Token::Number(4.0)
			]
		);
	}

	#[test]
	fn test_display() {
		let mut lexer = Lexer::new("-1.1 + x");
		assert_eq!(lexer.to_string(), "expression: \"-1.1 + x\", position: 0");
		lexer.next();
		assert_eq!(lexer.to_string(), "expression: \"-1.1 + x\", position: 1");
		lexer.next();
		assert_eq!(lexer.to_string(), "expression: \"-1.1 + x\", position: 4");
	}
}
