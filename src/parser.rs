use crate::lexer::Token;
use crate::ast::{Statement, Expr, Op};

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, pos: 0 }
	}

	fn peek(&self) -> &Token {
		self.tokens.get(self.pos).unwrap_or(&Token::Eof)
	}

	fn next(&mut self) -> &Token {
		let tok = self.tokens.get(self.pos).unwrap_or(&Token::Eof);
		self.pos += 1;
		tok
	}

	pub fn parse_statements(&mut self) -> Vec<Statement> {
		let mut stmts = Vec::new();
		while !matches!(self.peek(), Token::Eof) {
			match self.peek() {
				Token::ForEach | Token::FindFirst | Token::Create | Token::Delete => {
					if let Some(stmt) = self.parse_statement() {
						stmts.push(stmt);
					} else {
						self.next();
					}
				}
				Token::Display => {
					self.next();
					while !matches!(self.peek(), Token::Dot | Token::Eof) {
						self.next();
					}
					if let Token::Dot = self.peek() {
						self.next();
					}
				}
				_ => {
					self.next();
				}
			}
		}
		stmts
	}

	fn parse_statement(&mut self) -> Option<Statement> {
		match self.peek() {
			Token::ForEach => self.parse_foreach(),
			Token::FindFirst => self.parse_findfirst(),
			Token::Create => self.parse_create(),
			Token::Delete => self.parse_delete(),
			Token::Display => self.parse_display(),
			_ => None,
		}
	}

	fn parse_foreach(&mut self) -> Option<Statement> {
		self.next();
		let table = if let Token::Identifier(name) = self.next() {
			name.clone()
		} else {
			return None;
		};
		let mut where_clause = None;
		if let Token::Where = self.peek() {
			self.next();
			where_clause = self.parse_expr();
		}
		if let Token::Colon = self.peek() {
			self.next();
		}
		let mut body = Vec::new();
		while !matches!(self.peek(), Token::Identifier(s) if s == "END") && !matches!(self.peek(), Token::Eof) {
			if let Some(stmt) = self.parse_statement() {
				body.push(stmt);
			} else {
				self.next();
			}
		}
		if let Token::Identifier(s) = self.peek() {
			if s == "END" {
				self.next();
				if let Token::Dot = self.peek() {
					self.next();
				}
			}
		}
		Some(Statement::ForEach {
			table,
			where_clause,
			body,
		})
	}

	fn parse_findfirst(&mut self) -> Option<Statement> {
		self.next();
		let table = if let Token::Identifier(name) = self.next() {
			name.clone()
		} else {
			return None;
		};
		let mut where_clause = None;
		if let Token::Where = self.peek() {
			self.next();
			where_clause = self.parse_expr();
		}
		if let Token::Colon = self.peek() {
			self.next();
		}
		if let Token::Identifier(s) = self.peek() {
			if s == "END" {
				self.next();
				if let Token::Dot = self.peek() {
					self.next();
				}
			}
		}
		Some(Statement::FindFirst {
			table,
			where_clause,
		})
	}

	fn parse_create(&mut self) -> Option<Statement> {
		self.next();
		let table = if let Token::Identifier(name) = self.next() {
			name.clone()
		} else {
			return None;
		};
		if let Token::Dot = self.peek() {
			self.next();
		}
		Some(Statement::Create { table })
	}

	fn parse_delete(&mut self) -> Option<Statement> {
		self.next();
		let table = if let Token::Identifier(name) = self.next() {
			name.clone()
		} else {
			return None;
		};
		if let Token::Dot = self.peek() {
			self.next();
		}
		Some(Statement::Delete { table })
	}

	fn parse_display(&mut self) -> Option<Statement> {
		self.next();
		let mut fields = Vec::new();
		loop {
			match self.peek() {
				Token::Identifier(name) => {
					let name = name.clone();
					self.next();
					fields.push(name);
				}
				Token::StringLit(s) => {
					let s = format!("\"{}\"", s);
					self.next();
					fields.push(s);
				}
				Token::Number(n) => {
					fields.push(n.to_string());
					self.next();
				}
				Token::Float(f) => {
					fields.push(f.to_string());
					self.next();
				}
				_ => break,
			}
		}
		if let Token::Dot = self.peek() {
			self.next();
		}
		Some(Statement::Display { fields })
	}

	fn parse_expr(&mut self) -> Option<Expr> {
		let mut left = self.parse_primary()?;
		while let Some(op) = self.peek_op() {
			self.next();
			let right = self.parse_primary()?;
			left = Expr::BinOp {
				left: Box::new(left),
				op,
				right: Box::new(right),
			};
		}
		Some(left)
	}

	fn parse_primary(&mut self) -> Option<Expr> {
		match self.peek() {
			Token::LParen => {
				self.next();
				let expr = self.parse_expr()?;
				if let Token::RParen = self.peek() {
					self.next();
				}
				Some(Expr::Group(Box::new(expr)))
			}
			Token::Identifier(name) => {
				let mut full_name = name.clone();
				self.next();
				if let Token::Dot = self.peek() {
					if let Some(Token::Identifier(field)) = self.tokens.get(self.pos + 1) {
						full_name.push('.');
						full_name.push_str(field);
						self.next();
						self.next();
					}
				}
				Some(Expr::Identifier(full_name))
			}
			Token::StringLit(s) => {
				let s = s.clone();
				self.next();
				Some(Expr::String(s))
			}
			Token::Number(n) => {
				let n = *n;
				self.next();
				Some(Expr::Number(n))
			}
			Token::Float(f) => {
				let f = *f;
				self.next();
				Some(Expr::Float(f))
			}
			_ => None,
		}
	}

	fn peek_op(&self) -> Option<Op> {
		match self.peek() {
			Token::Equals => Some(Op::Eq),
			Token::NotEquals => Some(Op::Neq),
			Token::LessThan => Some(Op::Lt),
			Token::GreaterThan => Some(Op::Gt),
			Token::LessOrEqual => Some(Op::Le),
			Token::GreaterOrEqual => Some(Op::Ge),
			Token::And => Some(Op::And),
			Token::Or => Some(Op::Or),
			_ => None,
		}
	}
}
