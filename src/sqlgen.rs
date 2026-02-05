use crate::ast::{Statement, Expr, Op};

pub fn statement_to_sql(stmt: &Statement) -> Option<String> {
	match stmt {
		Statement::ForEach { table, where_clause, body } => {
			for s in body {
				if let Statement::Display { fields } = s {
					let literals_only = !fields.is_empty() && fields.iter().all(|f| f.starts_with('"') && f.ends_with('"'));
					if literals_only {
						let msg = fields.iter().map(|f| f.trim_matches('"')).collect::<Vec<_>>().join(" ");
						println!("{}", msg);
						return None;
					} else {
						let to_select: Vec<String> = fields.iter().filter(|f| !f.starts_with('"') && f.parse::<f64>().is_err()).cloned().collect();
						let fields_sql = if to_select.is_empty() { "*".to_string() } else { to_select.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>().join(", ") };
						let mut sql = format!("SELECT {} FROM \"{}\"", fields_sql, table);
						if let Some(expr) = where_clause {
							if let Some(w) = expr_to_sql(expr) {
								sql.push_str(&format!(" WHERE {}", w));
							}
						}
						return Some(sql);
					}
				}
			}
			let mut sql = format!("SELECT * FROM \"{}\"", table);
			if let Some(expr) = where_clause {
				if let Some(w) = expr_to_sql(expr) {
					sql.push_str(&format!(" WHERE {}", w));
				}
			}
			Some(sql)
		}
		Statement::FindFirst { table, where_clause } => {
			let mut sql = format!("SELECT * FROM \"{}\"", table);
			if let Some(expr) = where_clause {
				if let Some(w) = expr_to_sql(expr) {
					sql.push_str(&format!(" WHERE {}", w));
				}
			}
			sql.push_str(" LIMIT 1");
			Some(sql)
		}
		Statement::Create { table } => {
			Some(format!("INSERT INTO \"{}\" DEFAULT VALUES", table))
		}
		Statement::Delete { table } => {
			Some(format!("DELETE FROM \"{}\"", table))
		}
		Statement::Display { fields } => {
			if fields.iter().all(|f| f.starts_with('"') && f.ends_with('"')) {
				let msg = fields.iter().map(|f| f.trim_matches('"')).collect::<Vec<_>>().join(" ");
				println!("{}", msg);
				None
			} else {
				None
			}
		}
	}
}

pub fn expr_to_sql(expr: &Expr) -> Option<String> {
	match expr {
		Expr::Identifier(s) => Some(format!("\"{}\"", s)),
		Expr::String(s) => Some(format!("'{}'", s)),
		Expr::Number(n) => Some(n.to_string()),
		Expr::Float(f) => Some(f.to_string()),
		Expr::Group(expr) => {
			let inner = expr_to_sql(expr)?;
			Some(format!("({})", inner))
		}
		Expr::BinOp { left, op, right } => {
			let l = expr_to_sql(left)?;
			let r = expr_to_sql(right)?;
			match op {
				Op::Eq => Some(format!("{} = {}", l, r)),
				Op::Neq => Some(format!("{} <> {}", l, r)),
				Op::Lt => Some(format!("{} < {}", l, r)),
				Op::Gt => Some(format!("{} > {}", l, r)),
				Op::Le => Some(format!("{} <= {}", l, r)),
				Op::Ge => Some(format!("{} >= {}", l, r)),
				Op::And => Some(format!("{} AND {}", l, r)),
				Op::Or => Some(format!("{} OR {}", l, r)),
			}
		}
	}
}
