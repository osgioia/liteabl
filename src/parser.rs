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
		self.next(); // consume ForEach
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
			self.next(); // consume :
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
		self.next(); // consume FindFirst
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
			self.next(); // consume :
		}
		// Para PoC, ignorar body en FindFirst
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
		self.next(); // consume Create
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
		self.next(); // consume Delete
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
		self.next(); // consume Display
		let mut fields = Vec::new();
		while let Token::Identifier(_name) = self.peek() {
			if let Token::Identifier(name) = self.next() {
				fields.push(name.clone());
			}
		}
		if let Token::Dot = self.peek() {
			self.next();
		}
		Some(Statement::Display { fields })
	}

	fn parse_expr(&mut self) -> Option<Expr> {
		// Soporta solo igualdad para PoC: campo = valor
		let left = match self.peek() {
			Token::Identifier(name) => {
				let name = name.clone();
				self.next();
				Expr::Identifier(name)
			}
			Token::StringLit(s) => {
				let s = s.clone();
				self.next();
				Expr::String(s)
			}
			Token::Number(n) => {
				let n = *n;
				self.next();
				Expr::Number(n)
			}
			_ => return None,
		};
		if let Token::Equals = self.peek() {
			self.next();
			let right = match self.peek() {
				Token::Identifier(name) => {
					let name = name.clone();
					self.next();
					Expr::Identifier(name)
				}
				Token::StringLit(s) => {
					let s = s.clone();
					self.next();
					Expr::String(s)
				}
				Token::Number(n) => {
					let n = *n;
					self.next();
					Expr::Number(n)
				}
				_ => return None,
			};
			Some(Expr::BinOp {
				left: Box::new(left),
				op: Op::Eq,
				right: Box::new(right),
			})
		} else {
			Some(left)
		}
	}
}
